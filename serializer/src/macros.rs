//!
//! Macros for loading and storing items using Borsh and Serializer.
//!

/// Create Payload struct - a `#repr[transparent]` struct
/// wrapping `Cursor<Vec<u8>>`. This struct acts as a helper
/// for storing and loading items into a `Vec<u8>` buffer.
#[macro_export]
macro_rules! payload {
    () => {{
        $crate::payload::Payload::default()
    }};
    ($value:expr) => {{
        $crate::payload::Payload::with_capacity($value)
    }};
}

#[macro_export]
macro_rules! version {
    ($major:expr, $minor:expr) => {{
        $crate::payload::Version::new($major, $minor)
    }};
}

#[macro_export]
macro_rules! writer {
    ($value:expr) => {{
        ($value.as_mut() as &mut std::io::Cursor<Vec<u8>>)
    }};
}

#[macro_export]
macro_rules! reader {
    ($value:expr) => {{
        &mut std::io::Cursor::new($value.into_inner())
    }};
}

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
        $crate::payload::ser::Payload::<$type>($value).serialize($writer)
    };
}

/// Load item using Serializer deserialization. [`crate::serializer::Serializer`] is meant to provide
/// custom serialization over Borsh that can be used to store additional
/// metadata such as struct version.
#[macro_export]
macro_rules! deserialize {
    ($type:ty, $reader:expr) => {
        $crate::payload::de::Payload::<$type>::deserialize($reader).map(|x| x.into_inner())
    };
}
