//!

mod support;

#[doc(hidden)]
pub mod export {
    pub use serde::de::{Error, MapAccess, SeqAccess, Unexpected, Visitor};
    pub use serde::{Deserialize, Deserializer};

    pub use std::fmt;
    pub use std::marker::PhantomData;
    pub use std::option::Option::{None, Some};
    pub use std::result::Result::{self, Err, Ok};
}

use serde::{de::DeserializeSeed, Deserializer};
pub use serde_deserialize_over_derive::DeserializeOver;

pub trait DeserializeOver {
    fn deserialize_over<'de, D>(&mut self, de: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>;
}

pub trait DeserializeInto<'de>: Deserializer<'de> {
    fn deserialize_into<T: DeserializeOver>(self, target: &mut T) -> Result<(), Self::Error>;
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
