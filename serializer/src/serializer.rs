use borsh::{BorshDeserialize, BorshSerialize};
use std::ops::Deref;

pub type IoError = std::io::Error;
pub type IoErrorKind = std::io::ErrorKind;
pub type IoResult<T> = std::io::Result<T>;

/// `Serializable<T>` is a stop-gap between Borsh serialization
/// and the actual type `T` that needs to be serialized.
/// `T` must implement `Serializer` as opposed to Borsh traits,
/// while `Serializable<T>` implements Borsh traits.
/// This allows functions requiring generics to require `Serializable<T>`
/// (not Borsh traits) and still be able to serialize/deserialize `T`
/// using Borsh.
#[repr(transparent)]
pub struct Serializable<T>(pub T)
where
    T: Serializer;

impl<T> Serializable<T>
where
    T: Serializer,
{
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Serializable<T>
where
    T: Serializer,
{
    fn from(t: T) -> Self {
        Serializable(t)
    }
}

impl<T> Deref for Serializable<T>
where
    T: Serializer,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Serializable<T>
where
    T: Serializer,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> BorshSerialize for Serializable<T>
where
    T: Serializer,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.serialize(writer)
    }
}

impl<T> BorshDeserialize for Serializable<T>
where
    T: Serializer,
{
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Serializable(T::deserialize(buf)?))
    }
}

pub trait Serializer: Sized {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self>;
}
