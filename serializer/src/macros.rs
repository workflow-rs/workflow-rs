//!
//! Macros for loading and storing items using Borsh and Serializer.
//!

/// Store item using Borsh serialization
#[macro_export]
macro_rules! store {
    ($type:ty, $value:expr, $writer:expr) => {
        <$type as borsh::BorshSerialize>::serialize($value, $writer)
    };
}

/// Load item using Borsh deserialization
#[macro_export]
macro_rules! load {
    ($type:ty, $reader:expr) => {
        <$type as borsh::BorshDeserialize>::deserialize_reader($reader)
    };
}

/// Store item using Serializer serialization. [`crate::serializer::Serializer`] is meant to provide
/// custom serialization over Borsh that can be used to store additional
/// metadata such as struct version.
#[macro_export]
macro_rules! serialize {
    ($type:ty, $value:expr, $writer:expr) => {
        <$type as $crate::serializer::Serializer>::serialize($value, $writer)
    };
}

/// Load item using Serializer deserialization. [`crate::serializer::Serializer`] is meant to provide
/// custom serialization over Borsh that can be used to store additional
/// metadata such as struct version.
#[macro_export]
macro_rules! deserialize {
    ($type:ty, $reader:expr) => {
        <$type as $crate::serializer::Serializer>::deserialize($reader)
    };
}
