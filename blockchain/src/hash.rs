use chrono::DateTime;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use sha2::{
    digest::generic_array::{sequence::GenericSequence, GenericArray},
    Digest, Sha256,
};
use std::{cmp::PartialEq, fmt::Formatter, ops::Deref};

#[derive(Debug, Clone)]
pub struct Hash(GenericArray<u8, <Sha256 as Digest>::OutputSize>);

impl Hash {
    pub fn new<'de, T, Z>(
        index: &u64,
        previous_hash: &Hash,
        timestamp: &DateTime<Z>,
        data: &T,
    ) -> Self
    where
        T: Serialize + Deserialize<'de>,
        Z: chrono::TimeZone,
    {
        let mut hasher = Sha256::new();
        hasher.input(index.to_string());
        hasher.input(previous_hash.as_ref());
        hasher.input(timestamp.timestamp().to_string());
        hasher.input(serde_json::to_string(data).unwrap());

        Hash(hasher.result())
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Hash {}

impl Default for Hash {
    fn default() -> Self {
        let empty_array = GenericArray::generate(|_| 0u8);

        Hash(empty_array)
    }
}

impl Deref for Hash {
    type Target = GenericArray<u8, <Sha256 as Digest>::OutputSize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&data_encoding::HEXLOWER.encode(&self))
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HashVisitor)
    }
}

struct HashVisitor;

impl<'de> Visitor<'de> for HashVisitor {
    type Value = Hash;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a hex string of length 64")
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value.len() != 64 {
            return Err(E::invalid_length(value.len(), &"64"));
        }

        let mut result = GenericArray::default();

        if let Err(e) = data_encoding::HEXLOWER.decode_mut(value.as_bytes(), &mut result) {
            return Err(E::custom(e.error));
        };

        Ok(Hash(result))
    }
}
