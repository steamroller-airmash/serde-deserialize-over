//! Implementations for types within std

mod array;
mod map;
mod option;
mod tuple;

use crate::DeserializeOver;
use serde::de::{DeserializeSeed, Deserializer, Deserialize};

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

pub struct DeserializeWrapper<'a, T>(pub &'a mut T);

impl<'a, 'de, T> DeserializeSeed<'de> for DeserializeWrapper<'a, T>
where
  T: Deserialize<'de>
{
  type Value = ();

  fn deserialize<D>(self, de: D) -> Result<Self::Value, D::Error>
  where
    D: Deserializer<'de>,
  {
    *self.0 = T::deserialize(de)?;
    Ok(())
  }
}
