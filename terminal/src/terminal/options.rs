//!
//! Terminal creation options
//!

use web_sys::Element;

/// Indicates the target element to which the Terminal instance should be
/// bound to in DOM (WASM-browser only)
pub enum TargetElement {
    /// Bind to the document body
    Body,
    /// Bind to a specific supplied [`web_sys::Element`]
    Element(Element),
    /// Bind to the element with a specific tag name
    TagName(String),
    /// Bind to the element with a specific id
    Id(String),
}

/// Terminal options
pub struct Options {
    /// Default prompt (string such as `"$ "`)
    pub prompt: Option<String>,
    /// Target DOM element (when running under WASM)
    pub element: TargetElement,
    /// Disable internal clipboard handling
    /// (useful when using clipboard API calls externally)
    pub disable_clipboard_handling: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            prompt: None,
            element: TargetElement::Body,
            disable_clipboard_handling : false
        }
    }
}

impl Options {
    /// Create new default options
    pub fn new() -> Options {
        Options::default()
    }

    /// Set prompt string
    pub fn with_prompt(mut self, prompt: &str) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Set target element
    pub fn with_element(mut self, element: TargetElement) -> Self {
        self.element = element;
        self
    }

    /// Get prompt string
    pub fn prompt(&self) -> String {
        self.prompt.as_ref().unwrap_or(&"$ ".to_string()).clone()
    }
}
