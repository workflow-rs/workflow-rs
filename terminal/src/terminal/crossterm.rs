use crate::Result;
use crate::keys::Key;
use crate::terminal::Options;
use crate::terminal::Terminal;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
pub use crossterm::terminal::disable_raw_mode;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::io::{Stdout, Write, stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

///
/// # Crossterm
///
/// Wrapper around Crossterm interface - [https://crates.io/crates/crossterm](https://crates.io/crates/crossterm)
///
pub struct Crossterm {
    terminal: Arc<Mutex<Option<Arc<Terminal>>>>,
    terminate: Arc<AtomicBool>,
    stdout: Arc<Mutex<Option<Stdout>>>,
}

impl Crossterm {
    /// Creates a new crossterm backend with default [`Options`].
    pub fn try_new() -> Result<Self> {
        Self::try_new_with_options(&Options::default())
    }
    /// Creates a new crossterm backend configured from the given [`Options`].
    pub fn try_new_with_options(_options: &Options) -> Result<Self> {
        let crossterm = Crossterm {
            terminal: Arc::new(Mutex::new(None)),
            terminate: Arc::new(AtomicBool::new(false)),
            stdout: Arc::new(Mutex::new(Some(stdout()))),
            // stdout: Arc::new(Mutex::new(Some(stdout().into_raw_mode().unwrap()))),
        };
        Ok(crossterm)
    }

    /// Binds this crossterm backend to the given [`Terminal`].
    pub async fn init(self: &Arc<Self>, terminal: &Arc<Terminal>) -> Result<()> {
        *self.terminal.lock().unwrap() = Some(terminal.clone());
        Ok(())
    }

    /// Signals the input loop to terminate at the next opportunity.
    pub fn exit(&self) {
        self.terminate.store(true, Ordering::SeqCst);
    }

    /// Returns the associated [`Terminal`]; panics if [`init`](Self::init) has not been called.
    pub fn terminal(&self) -> Arc<Terminal> {
        self.terminal.lock().unwrap().as_ref().unwrap().clone()
    }

    /// Enables raw mode, processes key input until termination, then restores normal mode.
    pub async fn run(&self) -> Result<()> {
        terminal::enable_raw_mode()?;
        self.flush();
        self.intake(&self.terminate).await?;
        self.flush();
        terminal::disable_raw_mode()?;

        Ok(())
    }

    /// Runs the blocking key-event loop, translating crossterm key events into
    /// [`Key`]s and feeding them to the terminal until `terminate` is set.
    pub async fn intake(&self, terminate: &Arc<AtomicBool>) -> Result<()> {
        loop {
            let event = event::read()?;
            // println!("{:?}",event);
            if let Event::Key(key) = event
                && matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat)
            {
                let key = match key.code {
                    KeyCode::Char(c) => {
                        if key.modifiers & KeyModifiers::ALT == KeyModifiers::ALT {
                            Key::Alt(c)
                        } else if key.modifiers & KeyModifiers::CONTROL == KeyModifiers::CONTROL {
                            Key::Ctrl(c)
                        } else {
                            Key::Char(c)
                        }
                    }
                    KeyCode::Enter => Key::Enter,
                    KeyCode::Esc => Key::Esc,
                    KeyCode::Left => Key::ArrowLeft,
                    KeyCode::Right => Key::ArrowRight,
                    KeyCode::Up => Key::ArrowUp,
                    KeyCode::Down => Key::ArrowDown,
                    KeyCode::Backspace => Key::Backspace,
                    _ => {
                        continue;
                    }
                };

                self.terminal().ingest(key, "".to_string()).await?;
                self.flush();

                if terminate.load(Ordering::SeqCst) {
                    break;
                }
            }
        }

        Ok(())
    }

    /// Prints the given value to stdout and flushes immediately.
    pub fn write<S>(&self, s: S)
    where
        S: ToString,
    {
        print!("{}", s.to_string());
        self.flush();
    }

    /// Flushes any buffered output to stdout.
    pub fn flush(&self) {
        // stdout
        if let Some(stdout) = self.stdout.lock().unwrap().as_mut() {
            stdout.flush().unwrap();
        }
    }
}

use std::{panic, process};

/// configure custom panic hook that disables terminal raw mode
/// supply a closure that will be called when a panic occurs
/// giving an opportunity to output a custom message.  The closure
/// should return a desirable process exit code.
pub fn init_panic_hook<F>(f: F)
where
    F: Fn() -> i32 + Send + Sync + 'static,
{
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        disable_raw_mode().ok();
        default_hook(panic_info);
        let exit_code = f();
        process::exit(exit_code);
    }));
}

// compatibility functions
impl Crossterm {
    /// Compatibility stub; font sizing is not supported under crossterm, always returns `None`.
    pub fn get_font_size(&self) -> Result<Option<f64>> {
        Ok(None)
    }

    /// Compatibility no-op; font sizing is not supported under crossterm.
    pub fn set_font_size(&self, _font_size: f64) -> Result<()> {
        Ok(())
    }

    /// Compatibility stub; the terminal column count is not tracked under crossterm, always returns `None`.
    pub fn cols(&self) -> Option<usize> {
        None
    }

    /// Compatibility stub; the terminal row count is not tracked under crossterm, always returns `None`.
    pub fn rows(&self) -> Option<usize> {
        None
    }

    /// Compatibility no-op; font sizing is not supported under crossterm, always returns `None`.
    pub fn increase_font_size(&self) -> Result<Option<f64>> {
        Ok(None)
    }

    /// Compatibility no-op; font sizing is not supported under crossterm, always returns `None`.
    pub fn decrease_font_size(&self) -> Result<Option<f64>> {
        Ok(None)
    }
}
