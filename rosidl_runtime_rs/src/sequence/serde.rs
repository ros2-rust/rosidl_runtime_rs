use serde::{de::Error, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

use super::{BoundedPrimitiveSequence, BoundedSequence, PrimitiveSequence, Sequence};
use crate::traits::{PrimitiveSequenceAlloc, SequenceAlloc};

impl<'de, T: Deserialize<'de> + SequenceAlloc> Deserialize<'de> for Sequence<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<_> = Deserialize::deserialize(deserializer)?;
        Ok(Self::from(v))
    }
}

impl<T: Serialize + SequenceAlloc> Serialize for Sequence<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self.iter() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<'de, T: Deserialize<'de> + SequenceAlloc, const N: usize> Deserialize<'de>
    for BoundedSequence<T, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<_> = Deserialize::deserialize(deserializer)?;
        Self::try_from(v).map_err(D::Error::custom)
    }
}

impl<T: Serialize + SequenceAlloc, const N: usize> Serialize for BoundedSequence<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de> + PrimitiveSequenceAlloc> Deserialize<'de> for PrimitiveSequence<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values: Vec<_> = Deserialize::deserialize(deserializer)?;
        Ok(Self::from(values))
    }
}

impl<T: Serialize + PrimitiveSequenceAlloc> Serialize for PrimitiveSequence<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self.iter() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<'de, T: Deserialize<'de> + PrimitiveSequenceAlloc, const N: usize> Deserialize<'de>
    for BoundedPrimitiveSequence<T, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values: Vec<_> = Deserialize::deserialize(deserializer)?;
        Self::try_from(values).map_err(D::Error::custom)
    }
}

impl<T: Serialize + PrimitiveSequenceAlloc, const N: usize> Serialize
    for BoundedPrimitiveSequence<T, N>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use crate::{BoundedPrimitiveSequence, PrimitiveSequence};

    quickcheck! {
        fn test_json_roundtrip_sequence(xs: PrimitiveSequence<i32>) -> bool {
            let value = serde_json::to_value(xs.clone()).unwrap();
            let recovered = serde_json::from_value(value).unwrap();
            xs == recovered
        }
    }

    quickcheck! {
        fn test_json_roundtrip_bounded_sequence(xs: BoundedPrimitiveSequence<i32, 256>) -> bool {
            let value = serde_json::to_value(xs.clone()).unwrap();
            let recovered = serde_json::from_value(value).unwrap();
            xs == recovered
        }
    }
}
