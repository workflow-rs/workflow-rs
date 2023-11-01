#[derive(Default, Debug, Clone)]
pub struct UnicodeString(pub Vec<char>);

impl std::fmt::Display for UnicodeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().collect();
        write!(f, "{}", s)
    }
}

impl UnicodeString {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    pub fn remove(&mut self, index: usize) -> char {
        self.0.remove(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_not_empty(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn push(&mut self, c: char) {
        self.0.push(c);
    }

    pub fn insert_char(&mut self, index: usize, c: char) {
        self.0.insert(index, c);
    }

    pub fn insert(&mut self, index: usize, us: UnicodeString) {
        self.0.splice(index..index, us.0);
    }

    pub fn extend(&mut self, us: UnicodeString) {
        self.0.extend(us.0);
    }

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
