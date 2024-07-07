pub mod error;
mod imports;
pub mod result;

pub mod action;
pub mod arglist;
pub mod format;
pub mod ip;
pub mod version;

pub mod prelude {
    pub use crate::action;
    pub use crate::arglist;
    pub use crate::format;
    pub use crate::ip;
    pub use crate::version;
}
