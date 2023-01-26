use crate::keys::Key;
use crate::terminal::Options;
use crate::terminal::Terminal;
use crate::Result;
use std::io::{stdin, stdout, Stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use termion::event::Key as K;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

///
/// # Termion
///
/// Wrapper around Termion interface - [https://crates.io/crates/termion](https://crates.io/crates/termion)
///
pub struct Termion {
    terminal: Arc<Mutex<Option<Arc<Terminal>>>>,
    terminate: Arc<AtomicBool>,
    stdout: Arc<Mutex<Option<RawTerminal<Stdout>>>>,
}

impl Termion {
    pub fn try_new() -> Result<Self> {
        Self::try_new_with_options(&Options::default())
    }
    pub fn try_new_with_options(_options: &Options) -> Result<Self> {
        let termion = Termion {
            terminal: Arc::new(Mutex::new(None)),
            terminate: Arc::new(AtomicBool::new(false)),
            stdout: Arc::new(Mutex::new(Some(stdout().into_raw_mode().unwrap()))),
        };
        Ok(termion)
    }

    pub async fn init(self: &Arc<Self>, terminal: &Arc<Terminal>) -> Result<()> {
        *self.terminal.lock().unwrap() = Some(terminal.clone());
        Ok(())
    }

    pub fn exit(&self) {
        self.terminate.store(true, Ordering::SeqCst);
    }

    pub fn terminal(&self) -> Arc<Terminal> {
        self.terminal.lock().unwrap().as_ref().unwrap().clone()
    }

    pub async fn run(&self) -> Result<()> {
        self.flush();
        self.intake(&self.terminate).await?;
        self.flush();
        self.stdout
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .suspend_raw_mode()
            .unwrap();
        *self.stdout.lock().unwrap() = None;
        Ok(())
    }

    pub async fn intake(&self, terminate: &Arc<AtomicBool>) -> Result<()> {
        let stdin = stdin();
        for c in stdin.keys() {
            let key = match c.unwrap() {
                // K::Char('q') => break,
                K::Char(c) => {
                    if c == '\n' || c == '\r' {
                        Key::Enter
                    } else {
                        Key::Char(c)
                    }
                }
                K::Alt(c) => Key::Alt(c),
                K::Ctrl(c) => Key::Ctrl(c),
                K::Esc => Key::Esc,
                K::Left => Key::ArrowLeft,
                K::Right => Key::ArrowRight,
                K::Up => Key::ArrowUp,
                K::Down => Key::ArrowDown,
                K::Backspace => Key::Backspace,
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

        Ok(())
    }

    pub fn write<S>(&self, s: S)
    where
        S: Into<String>,
    {
        print!("{}", s.into());
        self.flush();
    }

    pub fn flush(&self) {
        if let Some(stdout) = self.stdout.lock().unwrap().as_mut() {
            stdout.flush().unwrap();
        }
    }
}
