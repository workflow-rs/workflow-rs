/// A character-indexed string stored as a vector of [`char`]s, allowing
/// indexing and editing by Unicode scalar value rather than by byte offset.
#[derive(Default, Debug, Clone)]
pub struct UnicodeString(pub Vec<char>);

impl std::fmt::Display for UnicodeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().collect();
        write!(f, "{}", s)
    }
}

impl UnicodeString {
    /// Removes all characters, leaving the string empty.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Removes and returns the last character, or `None` if the string is empty.
    pub fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    /// Removes and returns the character at the given index, shifting subsequent characters left.
    pub fn remove(&mut self, index: usize) -> char {
        self.0.remove(index)
    }

    /// Returns the number of characters in the string.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the string contains no characters.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the string contains at least one character.
    pub fn is_not_empty(&self) -> bool {
        !self.0.is_empty()
    }

    /// Appends a single character to the end of the string.
    pub fn push(&mut self, c: char) {
        self.0.push(c);
    }

    /// Inserts a single character at the given character index, shifting subsequent characters right.
    pub fn insert_char(&mut self, index: usize, c: char) {
        self.0.insert(index, c);
    }

    /// Inserts all characters of `us` at the given character index, shifting subsequent characters right.
    pub fn insert(&mut self, index: usize, us: UnicodeString) {
        self.0.splice(index..index, us.0);
    }

    /// Appends all characters of `us` to the end of this string.
    pub fn extend(&mut self, us: UnicodeString) {
        self.0.extend(us.0);
    }

    /// Returns an iterator over the characters in the string.
    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.0.iter()
    }
}

impl From<Vec<char>> for UnicodeString {
    fn from(v: Vec<char>) -> Self {
        Self(v)
    }
}

impl From<&[char]> for UnicodeString {
    fn from(v: &[char]) -> Self {
        Self(v.to_vec())
    }
}

impl From<&str> for UnicodeString {
    fn from(s: &str) -> Self {
        Self(s.chars().collect())
    }
}

impl From<String> for UnicodeString {
    fn from(s: String) -> Self {
        Self(s.chars().collect())
    }
}
