//!
//! Terminal key definitions
//!

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// A key press received from the terminal, normalized across platforms.
pub enum Key {
    /// The Enter (Return) key.
    Enter,
    /// The Backspace key, deleting the character before the cursor.
    Backspace,
    /// The Left arrow key.
    ArrowLeft,
    /// The Right arrow key.
    ArrowRight,
    /// The Up arrow key.
    ArrowUp,
    /// The Down arrow key.
    ArrowDown,
    /// The Home key.
    Home,
    /// The End key.
    End,
    /// The Page Up key.
    PageUp,
    /// The Page Down key.
    PageDown,
    /// A Shift+Tab (back-tab) press, moving focus backward.
    BackTab,
    /// The Delete key, deleting the character at the cursor.
    Delete,
    /// The Insert key.
    Insert,
    /// A printable character key.
    Char(char),
    /// A character pressed together with the Alt modifier.
    Alt(char),
    /// A character pressed together with the Ctrl modifier.
    Ctrl(char),
    /// The Escape key.
    Esc,
}
