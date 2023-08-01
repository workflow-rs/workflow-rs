//!
//! Module implementing the terminal interface abstraction
//!

use crate::clear::*;
use crate::cli::Cli;
use crate::cursor::*;
use crate::error::Error;
use crate::keys::Key;
use crate::result::Result;
use crate::CrLf;
use cfg_if::cfg_if;
use futures::*;
pub use pad::PadStr;
use regex::Regex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use workflow_core::channel::{unbounded, Channel, DuplexChannel, Receiver, Sender};
use workflow_core::task::spawn;
use workflow_log::log_error;

const DEFAULT_PARA_WIDTH: usize = 80;

pub struct Modifiers {
    pub alt: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
}
pub type LinkMatcherHandlerFn = Arc<Box<(dyn Fn(Modifiers, &str))>>;

#[derive(Debug, Clone)]
pub enum Event {
    Copy,
    Paste,
}
pub type EventHandlerFn = Arc<Box<(dyn Fn(Event))>>;

mod options;
pub use options::Options;
pub use options::TargetElement;

pub mod bindings;
pub mod xterm;
pub use xterm::{Theme, ThemeOption};

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        // pub mod xterm;
        // pub mod bindings;
        use crate::terminal::xterm::Xterm as Interface;

    } else if #[cfg(termion)] {
        pub mod termion;
        use crate::terminal::termion::Termion as Interface;
    } else {
        pub mod crossterm;
        use crate::terminal::crossterm::Crossterm as Interface;
        pub use crate::terminal::crossterm::{disable_raw_mode,init_panic_hook};
    }
}

#[derive(Debug)]
pub struct Inner {
    pub buffer: String,
    history: Vec<String>,
    pub cursor: usize,
    history_index: usize,
}

impl Default for Inner {
    fn default() -> Self {
        Self::new()
    }
}

impl Inner {
    pub fn new() -> Self {
        Inner {
            buffer: String::new(),
            history: Vec::new(),
            cursor: 0,
            history_index: 0,
        }
    }

    pub fn reset_line_buffer(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }
}

#[derive(Clone)]
struct UserInput {
    prompt: Arc<Mutex<String>>,
    buffer: Arc<Mutex<String>>,
    enabled: Arc<AtomicBool>,
    echo: Arc<AtomicBool>,
    kbhit: Arc<AtomicBool>,
    terminate: Arc<AtomicBool>,
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl UserInput {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        UserInput {
            prompt: Arc::new(Mutex::new(String::new())),
            buffer: Arc::new(Mutex::new(String::new())),
            enabled: Arc::new(AtomicBool::new(false)),
            echo: Arc::new(AtomicBool::new(false)),
            kbhit: Arc::new(AtomicBool::new(false)),
            terminate: Arc::new(AtomicBool::new(false)),
            sender,
            receiver,
        }
    }

    pub fn get_prompt(&self) -> String {
        self.prompt.lock().unwrap().clone()
    }

    pub fn get_buffer(&self) -> String {
        self.buffer.lock().unwrap().clone()
    }

    pub fn open(&self, echo: bool, kbhit: bool, prompt: String) -> Result<()> {
        *self.prompt.lock().unwrap() = prompt;
        self.enabled.store(true, Ordering::SeqCst);
        self.echo.store(echo, Ordering::SeqCst);
        self.kbhit.store(kbhit, Ordering::SeqCst);
        self.terminate.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        let s = {
            self.prompt.lock().unwrap().clear();
            let mut buffer = self.buffer.lock().unwrap();
            let s = buffer.clone();
            buffer.clear();
            s
        };

        self.enabled.store(false, Ordering::SeqCst);
        self.terminate.store(true, Ordering::SeqCst);
        self.sender.try_send(s).unwrap();
        Ok(())
    }

    pub async fn capture(
        &self,
        echo: bool,
        kbhit: bool,
        prompt: String,
        term: &Arc<Terminal>,
    ) -> Result<String> {
        self.open(echo, kbhit, prompt)?;

        let term = term.clone();
        let terminate = self.terminate.clone();

        cfg_if! {

            // TODO - refactor
            // this is currently a workaround due to DOM
            // clipboard API using JsPromise.
            if #[cfg(target_arch = "wasm32")] {
                workflow_core::task::dispatch(async move {
                    let _result = term.term().intake(&terminate).await;
                });
            } else {
                workflow_core::task::spawn(async move {
                    let _result = term.term().intake(&terminate).await;
                });
            }
        }

        let string = self.receiver.recv().await?;
        Ok(string)
    }

    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    fn is_echo(&self) -> bool {
        self.echo.load(Ordering::SeqCst)
    }

    fn is_kbhit(&self) -> bool {
        self.kbhit.load(Ordering::SeqCst)
    }

    fn inject(&self, key: Key, term: &Arc<Terminal>) -> Result<()> {
        match key {
            Key::Ctrl('c') => {
                self.close()?;
                term.abort();
            }
            Key::Char(ch) => {
                self.buffer.lock().unwrap().push(ch);
                if !self.is_echo() {
                    term.write(ch);
                }
                if self.is_kbhit() {
                    term.crlf();
                    self.close()?;
                }
            }
            Key::Backspace => {
                self.buffer.lock().unwrap().pop();
                if !self.is_echo() {
                    term.write("\x08 \x08");
                }
            }
            Key::Enter => {
                // term.writeln("");
                term.crlf();
                self.close()?;
            }
            _ => {}
        }
        Ok(())
    }
}

/// Terminal interface
#[derive(Clone)]
pub struct Terminal {
    inner: Arc<Mutex<Inner>>,
    pub running: Arc<AtomicBool>,
    pub prompt: Arc<Mutex<String>>,
    pub term: Arc<Interface>,
    pub handler: Arc<dyn Cli>,
    pub terminate: Arc<AtomicBool>,
    user_input: UserInput,
    pub pipe_raw: Channel<String>,
    pub pipe_crlf: Channel<String>,
    pub pipe_ctl: DuplexChannel<()>,
    pub para_width: Arc<AtomicUsize>,
}

impl Terminal {
    /// Create a new default terminal instance bound to the supplied command-line processor [`Cli`].
    pub fn try_new(handler: Arc<dyn Cli>, prompt: &str) -> Result<Self> {
        let term = Arc::new(Interface::try_new()?);

        let terminal = Self {
            inner: Arc::new(Mutex::new(Inner::new())),
            running: Arc::new(AtomicBool::new(false)),
            prompt: Arc::new(Mutex::new(prompt.to_string())),
            term,
            handler,
            terminate: Arc::new(AtomicBool::new(false)),
            user_input: UserInput::new(),
            pipe_raw: Channel::unbounded(),
            pipe_crlf: Channel::unbounded(),
            pipe_ctl: DuplexChannel::oneshot(),
            para_width: Arc::new(AtomicUsize::new(DEFAULT_PARA_WIDTH)),
        };

        Ok(terminal)
    }

    /// Create a new terminal instance bound to the supplied command-line processor [`Cli`].
    /// Receives [`options::Options`] that allow terminal customization.
    pub fn try_new_with_options(
        handler: Arc<dyn Cli>,
        // prompt : &str,
        options: Options,
    ) -> Result<Self> {
        let term = Arc::new(Interface::try_new_with_options(&options)?);

        let terminal = Self {
            inner: Arc::new(Mutex::new(Inner::new())),
            running: Arc::new(AtomicBool::new(false)),
            prompt: Arc::new(Mutex::new(options.prompt())),
            term,
            handler,
            terminate: Arc::new(AtomicBool::new(false)),
            user_input: UserInput::new(),
            pipe_raw: Channel::unbounded(),
            pipe_crlf: Channel::unbounded(),
            pipe_ctl: DuplexChannel::oneshot(),
            para_width: Arc::new(AtomicUsize::new(DEFAULT_PARA_WIDTH)),
        };

        Ok(terminal)
    }

    /// Init the terminal instance
    pub async fn init(self: &Arc<Self>) -> Result<()> {
        self.term.init(self).await?;

        self.handler.clone().init(self)?;

        Ok(())
    }

    /// Access to the underlying terminal instance
    pub fn inner(&self) -> LockResult<MutexGuard<'_, Inner>> {
        self.inner.lock()
    }

    /// Get terminal command line history list as `Vec<String>`
    pub fn history(&self) -> Vec<String> {
        let data = self.inner().unwrap();
        data.history.clone()
    }

    pub fn reset_line_buffer(&self) {
        self.inner().unwrap().reset_line_buffer();
    }

    /// Get the current terminal prompt string
    pub fn get_prompt(&self) -> String {
        if let Some(prompt) = self.handler.prompt() {
            prompt
        } else {
            self.prompt.lock().unwrap().clone()
        }
    }

    /// Render the current prompt in the terminal
    pub fn prompt(&self) {
        let mut data = self.inner().unwrap();
        data.cursor = 0;
        data.buffer.clear();
        self.term().write(self.get_prompt());
    }

    /// Output CRLF sequence
    pub fn crlf(&self) {
        self.term().write("\n\r".to_string());
    }

    /// Write a string
    pub fn write<S>(&self, s: S)
    where
        S: Into<String>,
    {
        self.term().write(s.into());
    }

    /// Write a string ending with CRLF sequence
    pub fn writeln<S>(&self, s: S)
    where
        S: Into<String>,
    {
        if self.is_running() {
            if self.user_input.is_enabled() {
                self.write(format!("{}{}\n\r", ClearLine, s.into()));
                self.write(self.user_input.get_prompt());
                if !self.user_input.echo.load(Ordering::SeqCst) {
                    self.write(self.user_input.get_buffer());
                }
            } else {
                self.write(format!("{}\n\r", s.into()));
            }
        } else {
            self.write(format!("{}{}\n\r", ClearLine, s.into()));
            let data = self.inner().unwrap();
            let p = format!("{}{}", self.get_prompt(), data.buffer);
            self.write(p);
            let l = data.buffer.len() - data.cursor;
            for _ in 0..l {
                self.write("\x08".to_string());
            }
        }
    }

    /// Refreshes the prompt and the user input buffer. This function
    /// is useful when the prompt is handled externally and contains
    /// data that should be updated.
    pub fn refresh_prompt(&self) {
        self.write(format!("{}", ClearLine));
        let data = self.inner().unwrap();
        let p = format!("{}{}", self.get_prompt(), data.buffer);
        self.write(p);
        let l = data.buffer.len() - data.cursor;
        for _ in 0..l {
            self.write("\x08".to_string());
        }
    }

    pub fn para<S>(&self, text: S)
    where
        S: Into<String>,
    {
        let width: usize = self.para_width.load(Ordering::SeqCst);
        let options = textwrap::Options::new(width).line_ending(textwrap::LineEnding::CRLF);

        textwrap::wrap(text.into().as_str(), options)
            .into_iter()
            .for_each(|line| self.writeln(line));
    }

    pub fn para_with_options<'a, S, Opt>(&self, width_or_options: Opt, text: S)
    where
        S: Into<String>,
        Opt: Into<textwrap::Options<'a>>,
    {
        // use textwrap::wrap;

        textwrap::wrap(text.into().crlf().as_str(), width_or_options.into())
            .into_iter()
            .for_each(|line| self.writeln(line));
    }

    pub fn help<S: ToString, H: ToString>(
        &self,
        list: &[(S, H)],
        separator: Option<&str>,
    ) -> Result<()> {
        let mut list = list
            .iter()
            .map(|(verb, help)| (verb.to_string(), help.to_string()))
            .collect::<Vec<_>>();
        list.sort_by_key(|(verb, _)| verb.to_string());
        let len = list.iter().map(|(c, _)| c.len()).fold(0, |a, b| a.max(b)) + 2;
        self.writeln("");
        for (verb, help) in list {
            self.writeln(format!(
                "{:>4} {} {} {}",
                "",
                verb.pad_to_width(len),
                separator.unwrap_or(""),
                help
            ));
        }
        self.writeln("");

        Ok(())
    }

    /// Get a clone of Arc of the underlying terminal instance
    pub fn term(&self) -> Arc<Interface> {
        Arc::clone(&self.term)
    }

    async fn pipe_start(self: &Arc<Self>) -> Result<()> {
        let self_ = self.clone();
        spawn(async move {
            loop {
                select! {
                    _ = self_.pipe_ctl.request.receiver.recv().fuse() => {
                        break;
                    },
                    raw = self_.pipe_raw.receiver.recv().fuse() => {
                        raw.map(|text|self_.write(text)).unwrap_or_else(|err|log_error!("Error writing from raw pipe: {err}"));
                    },
                    text = self_.pipe_crlf.receiver.recv().fuse() => {
                        text.map(|text|self_.writeln(text)).unwrap_or_else(|err|log_error!("Error writing from crlf pipe: {err}"));
                    },
                }
            }

            self_
                .pipe_ctl
                .response
                .sender
                .send(())
                .await
                .unwrap_or_else(|err| log_error!("Error posting shutdown ctl: {err}"));
        });
        Ok(())
    }

    async fn pipe_stop(self: &Arc<Self>) -> Result<()> {
        self.pipe_ctl.signal(()).await?;
        Ok(())
    }

    fn pipe_abort(self: &Arc<Self>) -> Result<()> {
        self.pipe_ctl.request.try_send(())?;
        Ok(())
    }

    /// Execute the async terminal processing loop.
    /// Once started, it should be stopped using
    /// [`Terminal::exit`]
    pub async fn run(self: &Arc<Self>) -> Result<()> {
        // self.prompt();

        self.pipe_start().await?;
        self.term().run().await
    }

    /// Exits the async terminal processing loop (async fn)
    pub async fn exit(self: &Arc<Self>) {
        self.terminate.store(true, Ordering::SeqCst);
        self.pipe_stop().await.unwrap_or_else(|err| panic!("{err}"));
        self.term.exit();
    }

    /// Exits the async terminal processing loop (sync fn)
    pub fn abort(self: &Arc<Self>) {
        self.terminate.store(true, Ordering::SeqCst);
        self.pipe_abort().unwrap_or_else(|err| panic!("{err}"));
        self.term.exit();
    }

    /// Ask a question (input a string until CRLF).
    /// `secure` argument suppresses echoing of the
    /// user input (useful for password entry)
    pub async fn ask(self: &Arc<Terminal>, echo: bool, prompt: &str) -> Result<String> {
        self.reset_line_buffer();
        self.term().write(prompt.to_string());
        self.user_input
            .capture(echo, false, prompt.to_string(), self)
            .await
    }

    pub async fn kbhit(self: &Arc<Terminal>, prompt: &str) -> Result<String> {
        self.reset_line_buffer();
        self.term().write(prompt.to_string());
        self.user_input
            .capture(true, true, prompt.to_string(), self)
            .await
    }

    /// Inject a string into the current cursor position
    pub fn inject(&self, text: String) -> Result<()> {
        let mut data = self.inner()?;
        self.inject_impl(&mut data, text)?;
        Ok(())
    }

    fn inject_impl(&self, data: &mut Inner, text: String) -> Result<()> {
        let len = text.len();
        data.buffer.insert_str(data.cursor, &text);
        self.trail(data.cursor, &data.buffer, true, false, len);
        data.cursor += len;
        Ok(())
    }

    async fn ingest(self: &Arc<Terminal>, key: Key, _term_key: String) -> Result<()> {
        if self.user_input.is_enabled() {
            self.user_input.inject(key, self)?;
            return Ok(());
        }

        match key {
            Key::Backspace => {
                let mut data = self.inner()?;
                if data.cursor == 0 {
                    return Ok(());
                }
                self.write("\x08".to_string());
                data.cursor -= 1;
                let idx = data.cursor;
                data.buffer.remove(idx);
                self.trail(data.cursor, &data.buffer, true, true, 0);
            }
            Key::ArrowUp => {
                let mut data = self.inner()?;
                if data.history_index == 0 {
                    return Ok(());
                }
                let current_buffer = data.buffer.clone();
                let index = data.history_index;
                //log_trace!("ArrowUp: index {}, data.history.len(): {}", index, data.history.len());
                if data.history.len() <= index {
                    data.history.push(current_buffer);
                } else {
                    data.history[index] = current_buffer;
                }
                data.history_index -= 1;

                data.buffer = data.history[data.history_index].clone();
                self.write(format!("{}{}{}", ClearLine, self.get_prompt(), data.buffer));
                data.cursor = data.buffer.len();
            }
            Key::ArrowDown => {
                let mut data = self.inner()?;
                let len = data.history.len();
                if data.history_index >= len {
                    return Ok(());
                }
                let index = data.history_index;
                data.history[index] = data.buffer.clone();
                data.history_index += 1;
                if data.history_index == len {
                    data.buffer.clear();
                } else {
                    data.buffer = data.history[data.history_index].clone();
                }

                self.write(format!("{}{}{}", ClearLine, self.get_prompt(), data.buffer));
                data.cursor = data.buffer.len();
            }
            Key::ArrowLeft => {
                let mut data = self.inner()?;
                if data.cursor == 0 {
                    return Ok(());
                }
                data.cursor -= 1;
                self.write(Left(1));
            }
            Key::ArrowRight => {
                let mut data = self.inner()?;
                if data.cursor < data.buffer.len() {
                    data.cursor += 1;
                    self.write(Right(1));
                }
            }
            Key::Enter => {
                let cmd = {
                    let mut data = self.inner()?;
                    let buffer = data.buffer.clone();
                    let length = data.history.len();

                    data.buffer.clear();
                    data.cursor = 0;

                    if !buffer.is_empty() {
                        let cmd = buffer.clone();

                        if length == 0 || !data.history[length - 1].is_empty() {
                            data.history_index = length;
                        } else {
                            data.history_index = length - 1;
                        }
                        let index = data.history_index;
                        if length <= index {
                            data.history.push(buffer);
                        } else {
                            data.history[index] = buffer;
                        }
                        data.history_index += 1;

                        Some(cmd)
                    } else {
                        None
                    }
                };

                self.crlf();

                if let Some(cmd) = cmd {
                    self.running.store(true, Ordering::SeqCst);
                    self.exec(cmd).await.ok();
                    self.running.store(false, Ordering::SeqCst);
                } else {
                    self.prompt();
                }
            }
            Key::Alt(_c) => {
                return Ok(());
            }
            Key::Ctrl('c') => {
                cfg_if! {
                    if #[cfg(not(target_arch = "wasm32"))] {
                        self.exit().await;
                    }
                }
                return Ok(());
            }
            Key::Ctrl(_c) => {
                return Ok(());
            }
            Key::Char(ch) => {
                self.inject(ch.to_string())?;
            }
            _ => {
                return Ok(());
            }
        }

        Ok(())
    }

    fn trail(&self, cursor: usize, buffer: &str, rewind: bool, erase_last: bool, offset: usize) {
        let mut tail = buffer[cursor..].to_string();
        if erase_last {
            tail += " ";
        }
        self.write(&tail);
        if rewind {
            let mut l = tail.len();
            if offset > 0 {
                l -= offset;
            }
            for _ in 0..l {
                self.write("\x08"); // backspace
            }
        }
    }

    /// Indicates that the terminal has received command input
    /// and has not yet returned from the processing. This flag
    /// is set to true when delivering the user command to the
    /// [`Cli`] handler and is reset to false when the [`Cli`]
    /// handler returns.
    #[inline]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub async fn exec<S: ToString>(self: &Arc<Terminal>, cmd: S) -> Result<()> {
        if let Err(err) = self
            .handler
            .clone()
            .digest(self.clone(), cmd.to_string())
            .await
        {
            self.writeln(err);
        }
        if self.terminate.load(Ordering::SeqCst) {
            self.term().exit();
        } else {
            self.prompt();
        }
        Ok(())
    }

    pub fn set_theme(&self, _theme: Theme) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        self.term.set_theme(_theme)?;
        Ok(())
    }

    pub fn update_theme(&self) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        self.term.update_theme()?;
        Ok(())
    }

    pub fn clipboard_copy(&self) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        self.term.clipboard_copy()?;
        Ok(())
    }

    pub fn clipboard_paste(&self) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        self.term.clipboard_paste()?;
        Ok(())
    }

    pub fn increase_font_size(&self) -> Result<Option<f64>> {
        self.term.increase_font_size()
    }

    pub fn decrease_font_size(&self) -> Result<Option<f64>> {
        self.term.decrease_font_size()
    }

    pub fn set_font_size(&self, font_size: f64) -> Result<()> {
        self.term.set_font_size(font_size)
    }

    pub fn get_font_size(&self) -> Result<Option<f64>> {
        self.term.get_font_size()
    }

    pub fn cols(&self) -> Option<usize> {
        self.term.cols()
    }

    pub async fn select<T>(self: &Arc<Terminal>, prompt: &str, list: &[T]) -> Result<Option<T>>
    where
        T: std::fmt::Display + Clone, // + IdT + Clone + Send + Sync + 'static,
    {
        if list.is_empty() {
            Ok(None)
        } else if list.len() == 1 {
            Ok(list.first().cloned())
        } else {
            let mut selection = None;
            while selection.is_none() {
                list.iter().enumerate().for_each(|(seq, item)| {
                    self.writeln(format!("{seq}: {item}"));
                });

                let text = self
                    .ask(
                        false,
                        &format!("{prompt} [{}..{}] or <enter> to abort: ", 0, list.len() - 1),
                    )
                    .await?
                    .trim()
                    .to_string();
                if text.is_empty() {
                    self.writeln("aborting...");
                    return Err(Error::UserAbort);
                } else {
                    match text.parse::<usize>() {
                        Ok(seq) if seq < list.len() => selection = list.get(seq).cloned(),
                        _ => {}
                    };
                }
            }

            Ok(selection)
        }
    }

    pub fn register_event_handler(self: &Arc<Self>, _handler: EventHandlerFn) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        self.term.register_event_handler(_handler)?;
        Ok(())
    }

    pub fn register_link_matcher(
        &self,
        _regexp: &js_sys::RegExp,
        _handler: LinkMatcherHandlerFn,
    ) -> Result<()> {
        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                self.term.register_link_matcher(_regexp, _handler)?;
            }
        }
        Ok(())
    }
}

/// Utility function to strip multiple whitespaces and return a `Vec<String>`
pub fn parse(s: &str) -> Vec<String> {
    let regex = Regex::new(r"\s+").unwrap();
    let s = regex.replace_all(s.trim(), " ");
    s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>()
}
