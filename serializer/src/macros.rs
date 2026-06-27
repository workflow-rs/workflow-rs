//!
//! Macros for loading and storing items using Borsh and Serializer.
//!

/// Create Payload struct - a `#repr[transparent]` struct
/// wrapping `Cursor<Vec<u8>>`. This struct acts as a helper
/// for storing and loading items into a `Vec<u8>` buffer.
#[macro_export]
macro_rules! payload {
    () => {{ $crate::payload::Payload::default() }};
    ($value:expr_2021) => {{ $crate::payload::Payload::with_capacity($value) }};
}

/// Construct a `crate::payload::Version` from major and minor version numbers.
#[macro_export]
macro_rules! version {
    ($major:expr_2021, $minor:expr_2021) => {{ $crate::payload::Version::new($major, $minor) }};
}

/// Borrow a `crate::payload::Payload` as a mutable `Cursor<Vec<u8>>` writer.
#[macro_export]
macro_rules! writer {
    ($value:expr_2021) => {{ ($value.as_mut() as &mut std::io::Cursor<Vec<u8>>) }};
}

/// Consume a `crate::payload::Payload` and produce a `Cursor<Vec<u8>>` reader over its bytes.
#[macro_export]
macro_rules! reader {
    ($value:expr_2021) => {{ &mut std::io::Cursor::new($value.into_inner()) }};
}

/// Store item using Borsh serialization
#[macro_export]
macro_rules! store {
    ($type:ty, $value:expr_2021, $writer:expr_2021) => {
        <$type as borsh::BorshSerialize>::serialize($value, $writer)
    };
}

/// Load item using Borsh deserialization
#[macro_export]
macro_rules! load {
    ($type:ty, $reader:expr_2021) => {
        <$type as borsh::BorshDeserialize>::deserialize_reader($reader)
    };
}

/// Store item using Serializer serialization. [`crate::serializer::Serializer`] is meant to provide
/// custom serialization over Borsh that can be used to store additional
/// metadata such as struct version.
#[macro_export]
macro_rules! serialize {
    ($type:ty, $value:expr_2021, $writer:expr_2021) => {
        $crate::payload::ser::Payload::<$type>($value).serialize($writer)
    };
}

/// Load item using Serializer deserialization. [`crate::serializer::Serializer`] is meant to provide
/// custom serialization over Borsh that can be used to store additional
/// metadata such as struct version.
#[macro_export]
macro_rules! deserialize {
    ($type:ty, $reader:expr_2021) => {
        $crate::payload::de::Payload::<$type>::deserialize($reader).map(|x| x.into_inner())
    };
}
