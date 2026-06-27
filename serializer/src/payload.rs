use crate::serializer::{Deserializer, Serializer};
use crate::{load, store};
use std::io::Cursor;

/// Serialization-side payload wrappers.
pub mod ser {
    use super::*;

    /// Borrows a [`Serializer`] value and serializes it as a length-prefixed byte buffer.
    #[repr(transparent)]
    pub struct Payload<'a, T>(pub &'a T)
    where
        T: Serializer;

    impl<T> Serializer for Payload<'_, T>
    where
        T: Serializer,
    {
        fn serialize<W: std::io::Write>(&self, target: &mut W) -> std::io::Result<()> {
            let mut payload = Vec::<u8>::new();
            let mut writer = Cursor::new(&mut payload);
            self.0.serialize(&mut writer)?;
            store!(Vec<u8>, &payload, target)?;
            Ok(())
        }
    }
}

/// Deserialization-side payload wrappers.
pub mod de {
    use super::*;

    /// Wraps a [`Deserializer`] value decoded from a length-prefixed byte buffer.
    #[repr(transparent)]
    pub struct Payload<T>(pub T)
    where
        T: Deserializer;

    impl<T> Deserializer for Payload<T>
    where
        T: Deserializer,
    {
        fn deserialize<R: borsh::io::Read>(source: &mut R) -> std::io::Result<Self> {
            let payload = load!(Vec::<u8>, source)?;
            let mut reader = Cursor::new(payload);
            let inner = T::deserialize(&mut reader)?;
            Ok(Payload(inner))
        }
    }

    impl<T> Payload<T>
    where
        T: Deserializer,
    {
        /// Consumes the payload, returning the wrapped deserialized value.
        pub fn into_inner(self) -> T {
            self.0
        }
    }
}
