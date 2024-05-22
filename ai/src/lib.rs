pub mod error;
pub mod gpt;
mod imports;
pub mod result;

pub mod prelude {
    pub use crate::gpt::ChatGPT;
}
