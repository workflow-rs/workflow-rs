pub mod macros;
pub mod payload;
pub mod result;
pub mod serializer;
pub mod tests;

pub mod prelude {
    pub use crate::serializer::{Deserializer, Serializable, Serializer};
    pub use crate::{deserialize, load, payload, reader, serialize, store, version, writer};
    pub use borsh::{BorshDeserialize, BorshSerialize};
}

pub use borsh;
