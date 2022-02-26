//! Traits and macros for deserializing data onto an existing instance of a
//! struct via serde. It is somewhat like a more featureful version of adding
//! `#[serde::default(...)]` annotations everywhere except that it is able to
//! use runtime data instead of hardcoded defaults.
//! 
//! # Usage
//! 
//! # Example

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

pub trait DeserializeOver<'de> {
    fn deserialize_over<D>(&mut self, de: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>;
}

pub trait DeserializeInto<'de>: Deserializer<'de> {
    fn deserialize_into<T: DeserializeOver<'de>>(self, target: &mut T) -> Result<(), Self::Error>;
}

#[doc(hidden)]
pub struct DeserializeOverWrapper<'a, T>(pub &'a mut T);

impl<'a, 'de, T> DeserializeSeed<'de> for DeserializeOverWrapper<'a, T>
where
    T: DeserializeOver<'de>,
{
    type Value = ();

    fn deserialize<D>(self, de: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_over(de)
    }
}
