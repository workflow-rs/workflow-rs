use crate::payload::{de, ser};
use crate::{load, store};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

pub trait SerializerT: Serializer + Send + Sync {}
impl<T> SerializerT for T where T: Serializer + Send + Sync {}

pub trait DeserializerT: Deserializer + Send + Sync {}
impl<T> DeserializerT for T where T: Deserializer + Send + Sync {}

#[derive(Debug, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Serializable<T>(pub T)
where
    T: SerializerT + DeserializerT;

impl<T> Serializable<T>
where
    T: SerializerT + DeserializerT,
{
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Serializable<T>
where
    T: SerializerT + DeserializerT,
{
    fn from(t: T) -> Self {
        Serializable(t)
    }
}

impl<T> Deref for Serializable<T>
where
    T: SerializerT + DeserializerT,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Serializable<T>
where
    T: SerializerT + DeserializerT,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> BorshSerialize for Serializable<T>
where
    T: SerializerT + DeserializerT,
{
    fn serialize<W: std::io::Write>(&self, target: &mut W) -> std::io::Result<()> {
        ser::Payload(&self.0).serialize(target)?;
        Ok(())
    }
}

impl<T> BorshDeserialize for Serializable<T>
where
    T: SerializerT + DeserializerT,
{
    fn deserialize_reader<R: borsh::io::Read>(source: &mut R) -> std::io::Result<Self> {
        let t = de::Payload::<T>::deserialize(source)?;
        Ok(Serializable(t.into_inner()))
    }
}

/// `Serializer` is a trait that allows for data serialization and deserialization
/// similar to Borsh, but via a separate trait. This allows for serialization
/// of additional metadata while using underlying Borsh primitives. For example:
/// a struct can implement both Borsh and Serializer traits where Serializer
/// can store custom metadata (e.g. struct version) and then store the struct
/// using Borsh.  Both [`Serializer`] and Borsh are almost identical, where
/// [`Serializer`] is meant to signal intent for custom serialization.
/// [`Serializer`] is a complimentary trait for [`Serializable`] struct
/// and can be used to prevent direct Borsh serialization of a struct.
pub trait Serializer: Sized {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;

    fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.serialize(&mut buf)?;
        Ok(buf)
    }
}

pub trait Deserializer: Sized {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>;

    fn try_from_slice(slice: &[u8]) -> std::io::Result<Self> {
        let mut buf = slice;
        Self::deserialize(&mut buf)
    }
}

type ResultStatusTag = u8;
const RESULT_OK: ResultStatusTag = 0;
const RESULT_ERR: ResultStatusTag = 1;

impl<T, E> Serializer for Result<T, E>
where
    T: Serializer + 'static,
    E: std::fmt::Display + BorshSerialize + 'static,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Ok(t) => {
                store!(ResultStatusTag, &RESULT_OK, writer)?;
                ser::Payload(t).serialize(writer)?;
            }
            Err(e) => {
                store!(ResultStatusTag, &RESULT_ERR, writer)?;
                store!(E, e, writer)?;
            }
        }

        Ok(())
    }
}

impl<T, E> Deserializer for Result<T, E>
where
    T: Deserializer + 'static,
    E: std::fmt::Display + BorshDeserialize + 'static,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let tag = load!(ResultStatusTag, reader)?;
        match tag {
            RESULT_OK => {
                let t = de::Payload::<T>::deserialize(reader)?;
                Ok(Ok(t.into_inner()))
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
                ser::Payload(t).serialize(writer)?;
            }
            None => {
                store!(OptionStatusTag, &OPTION_NONE, writer)?;
            }
        }

        Ok(())
    }
}

impl<T> Deserializer for Option<T>
where
    T: Deserializer + 'static,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let tag = load!(OptionStatusTag, reader)?;
        match tag {
            OPTION_SOME => {
                let t = de::Payload::<T>::deserialize(reader)?;
                Ok(Some(t.into_inner()))
            }
            OPTION_NONE => Ok(None),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid Serializer Option tag",
            )),
        }
    }
}

impl<V> Serializer for Vec<V>
where
    V: Serializer,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u32, &(self.len() as u32), writer)?;

        for item in self.iter() {
            ser::Payload(item).serialize(writer)?;
        }

        Ok(())
    }
}

impl<V> Deserializer for Vec<V>
where
    V: Deserializer,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut vec = Vec::with_capacity(len as usize);

        for _ in 0..len {
            let item = de::Payload::<V>::deserialize(reader)?;
            vec.push(item.into_inner());
        }

        Ok(vec)
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
            ser::Payload(v).serialize(writer)?;
        }

        Ok(())
    }
}

impl<K, V> Deserializer for std::collections::HashMap<K, V>
where
    K: Deserializer + std::hash::Hash + Eq,
    V: Deserializer,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut map = std::collections::HashMap::new();

        for _ in 0..len {
            let k = K::deserialize(reader)?;
            let v = de::Payload::<V>::deserialize(reader)?;
            map.insert(k, v.into_inner());
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
            ser::Payload(item).serialize(writer)?;
        }

        Ok(())
    }
}

impl<T> Deserializer for std::collections::HashSet<T>
where
    T: Deserializer + Send + Sync + std::hash::Hash + Eq,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut set = std::collections::HashSet::new();

        for _ in 0..len {
            let item = de::Payload::<T>::deserialize(reader)?;
            set.insert(item.into_inner());
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
            ser::Payload(v).serialize(writer)?;
        }

        Ok(())
    }
}

impl<K, V> Deserializer for ahash::AHashMap<K, V>
where
    K: Deserializer + Send + Sync + std::hash::Hash + Eq,
    V: Deserializer + Send + Sync,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut map = ahash::AHashMap::new();

        for _ in 0..len {
            let k = K::deserialize(reader)?;
            let v = de::Payload::<V>::deserialize(reader)?;
            map.insert(k, v.into_inner());
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
            ser::Payload(item).serialize(writer)?;
        }

        Ok(())
    }
}

impl<T> Deserializer for ahash::AHashSet<T>
where
    T: Deserializer + std::hash::Hash + Eq,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut set = ahash::AHashSet::new();

        for _ in 0..len {
            let item = de::Payload::<T>::deserialize(reader)?;
            set.insert(item.into_inner());
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
            ser::Payload(v).serialize(writer)?;
        }

        Ok(())
    }
}

impl<K, V> Deserializer for std::collections::BTreeMap<K, V>
where
    K: Deserializer + Ord,
    V: Deserializer,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut map = std::collections::BTreeMap::new();

        for _ in 0..len {
            let k = K::deserialize(reader)?;
            let v = de::Payload::<V>::deserialize(reader)?;
            map.insert(k, v.into_inner());
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
            ser::Payload(item).serialize(writer)?;
        }

        Ok(())
    }
}

impl<T> Deserializer for std::collections::BTreeSet<T>
where
    T: Deserializer + Ord,
{
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len: u32 = load!(u32, reader)?;
        let mut set = std::collections::BTreeSet::new();

        for _ in 0..len {
            let item = de::Payload::<T>::deserialize(reader)?;
            set.insert(item.into_inner());
        }

        Ok(set)
    }
}
