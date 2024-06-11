use crate::{load, store};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

pub trait SerializerT: Serializer + Send + Sync {}
impl<T> SerializerT for T where T: Serializer + Send + Sync {}

/// `Serializable<T>` is a stop-gap between Borsh serialization
/// and the actual type `T` that needs to be serialized.
/// `T` must implement `Serializer` as opposed to Borsh traits,
/// while `Serializable<T>` implements Borsh traits.
/// This allows functions requiring Borsh serialization to accept
/// T that does not implement Borsh traits by wrapping it in
/// `Serializable<T>`.
///
/// Example:
/// ```ignore
///
/// struct MyStruct {
///    field: u32,
/// }
///
/// impl Serializer for MyStruct {
///     fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
///         store!(u32, &1, writer)?;
///         store!(u32, &self.field, writer)?;
///         Ok(())
///     }
///
///     fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
///         let _version = load!(u32, buf)?;
///         let field = load!(u32, buf)?;
///         Ok(Self { field })
///     }
/// }
///
/// fn send<T>(serializable: T) where T : BorshSerialize { ... }
///
/// fn sender() {
///     let my_struct = MyStruct { field: 42 };
///     send(Serializable(my_struct));
/// }
/// ```
///
#[derive(Debug, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Serializable<T>(pub T)
where
    T: SerializerT;

impl<T> Serializable<T>
where
    T: SerializerT,
{
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Serializable<T>
where
    T: SerializerT,
{
    fn from(t: T) -> Self {
        Serializable(t)
    }
}

impl<T> Deref for Serializable<T>
where
    T: SerializerT,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Serializable<T>
where
    T: SerializerT,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> BorshSerialize for Serializable<T>
where
    T: SerializerT,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.serialize(writer)
    }
}

impl<T> BorshDeserialize for Serializable<T>
where
    T: SerializerT,
{
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let t = T::deserialize(reader)?;
        Ok(Serializable(t))
    }

    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Serializable(T::deserialize(&mut *buf)?))
    }
}

pub trait Serializer: Sized {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>;

    fn try_from_slice(slice: &[u8]) -> std::io::Result<Self> {
        let mut buf = slice;
        Self::deserialize(&mut buf)
    }

    fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.serialize(&mut buf)?;
        Ok(buf)
    }
}

type ResultStatusTag = u8;
const RESULT_OK: ResultStatusTag = 0;
const RESULT_ERR: ResultStatusTag = 1;

impl<T, E> Serializer for Result<T, E>
where
    T: Serializer + 'static,
    E: std::fmt::Display + BorshSerialize + BorshDeserialize + 'static,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Ok(t) => {
                store!(ResultStatusTag, &RESULT_OK, writer)?;
                t.serialize(writer)?;
            }
            Err(e) => {
                store!(ResultStatusTag, &RESULT_ERR, writer)?;
                store!(E, e, writer)?;
            }
        }

        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let tag = load!(ResultStatusTag, reader)?;
        match tag {
            RESULT_OK => {
                let t = T::deserialize(reader)?;
                Ok(Ok(t))
            }
            RESULT_ERR => {
                let e = E::deserialize_reader(reader)?;
                Ok(Err(e))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid Serializer Result tag",
            )),
        }
    }
}

type OptionStatusTag = u8;
const OPTION_SOME: OptionStatusTag = 1;
const OPTION_NONE: OptionStatusTag = 0;

impl<T> Serializer for Option<T>
where
    T: Serializer + 'static,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Some(t) => {
                store!(OptionStatusTag, &OPTION_SOME, writer)?;
                t.serialize(writer)?;
            }
            None => {
                store!(OptionStatusTag, &OPTION_NONE, writer)?;
            }
        }

        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let tag = load!(OptionStatusTag, reader)?;
        match tag {
            OPTION_SOME => {
                let t = T::deserialize(reader)?;
                Ok(Some(t))
            }
            OPTION_NONE => Ok(None),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid Serializer Option tag",
            )),
        }
    }
}

impl<K, V> Serializer for std::collections::HashMap<K, V>
where
    K: Serializer + std::hash::Hash + Eq,
    V: Serializer,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u32, &(self.len() as u32), writer)?;

        for (k, v) in self.iter() {
            k.serialize(writer)?;
            v.serialize(writer)?;
        }

        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut map = std::collections::HashMap::new();

        for _ in 0..len {
            let k = K::deserialize(reader)?;
            let v = V::deserialize(reader)?;
            map.insert(k, v);
        }

        Ok(map)
    }
}

impl<T> Serializer for std::collections::HashSet<T>
where
    T: Serializer + Send + Sync + std::hash::Hash + Eq,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u32, &(self.len() as u32), writer)?;

        for item in self.iter() {
            item.serialize(writer)?;
        }

        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut set = std::collections::HashSet::new();

        for _ in 0..len {
            let item = T::deserialize(reader)?;
            set.insert(item);
        }

        Ok(set)
    }
}

impl<K, V> Serializer for ahash::AHashMap<K, V>
where
    K: Serializer + Send + Sync + std::hash::Hash + Eq,
    V: Serializer + Send + Sync,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u32, &(self.len() as u32), writer)?;

        for (k, v) in self.iter() {
            k.serialize(writer)?;
            v.serialize(writer)?;
        }

        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut map = ahash::AHashMap::new();

        for _ in 0..len {
            let k = K::deserialize(reader)?;
            let v = V::deserialize(reader)?;
            map.insert(k, v);
        }

        Ok(map)
    }
}

impl<T> Serializer for ahash::AHashSet<T>
where
    T: Serializer + std::hash::Hash + Eq,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u32, &(self.len() as u32), writer)?;

        for item in self.iter() {
            item.serialize(writer)?;
        }

        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut set = ahash::AHashSet::new();

        for _ in 0..len {
            let item = T::deserialize(reader)?;
            set.insert(item);
        }

        Ok(set)
    }
}

impl<K, V> Serializer for std::collections::BTreeMap<K, V>
where
    K: Serializer + Ord,
    V: Serializer,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u32, &(self.len() as u32), writer)?;

        for (k, v) in self.iter() {
            k.serialize(writer)?;
            v.serialize(writer)?;
        }

        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut map = std::collections::BTreeMap::new();

        for _ in 0..len {
            let k = K::deserialize(reader)?;
            let v = V::deserialize(reader)?;
            map.insert(k, v);
        }

        Ok(map)
    }
}

impl<T> Serializer for std::collections::BTreeSet<T>
where
    T: Serializer + Ord,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u32, &(self.len() as u32), writer)?;

        for item in self.iter() {
            item.serialize(writer)?;
        }

        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut set = std::collections::BTreeSet::new();

        for _ in 0..len {
            let item = T::deserialize(reader)?;
            set.insert(item);
        }

        Ok(set)
    }
}
