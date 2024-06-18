pub mod macros;
pub mod result;
pub mod serializer;
pub mod tests;

pub mod prelude {
    pub use crate::serializer::{Serializable, Serializer};
    pub use crate::{deserialize, load, serialize, store};
}
