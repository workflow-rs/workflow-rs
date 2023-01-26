//!
//! Terminal key definitions
//!

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Enter,
    Backspace,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Home,
    End,
    PageUp,
    PageDown,
    BackTab,
    Delete,
    Insert,
    Char(char),
    Alt(char),
    Ctrl(char),
    Esc,
}
