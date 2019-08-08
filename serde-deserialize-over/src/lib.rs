#[doc(hidden)]
pub use serde;

use serde::{de::DeserializeSeed, Deserializer};
pub use serde_deserialize_over_derive::DeserializeOver;

pub trait DeserializeOver {
    fn deserialize_over<'de, D>(&mut self, de: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>;
}

pub struct DeserializeOverWrapper<'a, T>(pub &'a mut T);

impl<'a, 'de, T> DeserializeSeed<'de> for DeserializeOverWrapper<'a, T>
where
    T: DeserializeOver,
{
    type Value = ();

    fn deserialize<D>(self, de: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_over(de)
    }
}
