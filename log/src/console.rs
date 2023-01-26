#[cfg(not(target_os = "solana"))]
pub use console::style;

#[cfg(target_os = "solana")]
mod console_style {
    use std::fmt::*;

    pub struct ConsoleStyle<'t>(pub &'t str);

    impl<'t> ConsoleStyle<'t> {
        pub fn black(&'t self) -> &'t Self {
            self
        }
        pub fn white(&'t self) -> &'t Self {
            self
        }
        pub fn red(&'t self) -> &'t Self {
            self
        }
        pub fn green(&'t self) -> &'t Self {
            self
        }
        pub fn blue(&'t self) -> &'t Self {
            self
        }
        pub fn yellow(&'t self) -> &'t Self {
            self
        }
        pub fn cyan(&'t self) -> &'t Self {
            self
        }
        pub fn magenta(&'t self) -> &'t Self {
            self
        }
        pub fn color256(&'t self) -> &'t Self {
            self
        }

        pub fn on_black(&'t self) -> &'t Self {
            self
        }
        pub fn on_white(&'t self) -> &'t Self {
            self
        }
        pub fn on_red(&'t self) -> &'t Self {
            self
        }
        pub fn on_green(&'t self) -> &'t Self {
            self
        }
        pub fn on_blue(&'t self) -> &'t Self {
            self
        }
        pub fn on_yellow(&'t self) -> &'t Self {
            self
        }
        pub fn on_cyan(&'t self) -> &'t Self {
            self
        }
        pub fn on_magenta(&'t self) -> &'t Self {
            self
        }
    }

    impl<'t> Display for ConsoleStyle<'t> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{}", self.0)
        }
    }

    // #[cfg(target_os = "solana")]
    // impl<'t> Into<&'t str> for ConsoleStyle<'t> {
    //     fn into(self) -> &'t str { self.text }
    // }
}

#[cfg(target_os = "solana")]
pub fn style<'t>(text: &'t str) -> console_style::ConsoleStyle<'t> {
    console_style::ConsoleStyle(text)
}
