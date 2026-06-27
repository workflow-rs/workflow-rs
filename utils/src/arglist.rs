use crate::imports::*;

/// Accumulator for command-line arguments; converting it into a `Vec<String>`
/// de-duplicates the collected entries.
#[derive(Default)]
pub struct Arglist {
    /// The arguments collected so far, in insertion order.
    pub args: Vec<String>,
}

impl Arglist {
    /// Appends an argument to the list.
    pub fn push(&mut self, arg: impl Into<String>) {
        self.args.push(arg.into());
    }
}

impl From<Arglist> for Vec<String> {
    fn from(arglist: Arglist) -> Self {
        let mut args = AHashSet::new();
        for arg in arglist.args.into_iter() {
            args.insert(arg);
        }
        args.into_iter().collect()
    }
}
