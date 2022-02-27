//! Traits and macros for deserializing data onto an existing instance of a
//! struct via serde. It is somewhat like a more featureful version of adding
//! `#[serde::default(...)]` annotations everywhere except that it is able to
//! use runtime data instead of hardcoded defaults.
//!
//! # Usage
//! The main trait for this crate is the [`DeserializeOver`] trait and its
//! corresponding derive macro. It works analogously to serde's [`Deserialize`]
//! trait except that struct fields that are not present within the data being
//! deserialized keep their values as is.
//!
//! For a simple struct, this ends up looking something like this:
//! ```
//! use serde_deserialize_over::DeserializeOver;
//! # use serde_json::Deserializer;
//! # use serde_json::de::StrRead;
//!
//! #[derive(DeserializeOver)]
//! struct MyStruct {
//!     pub a: String,
//!     pub b: i32
//! }
//!
//! let json = r#"{ "a": "test" }"#;
//! let mut inst = MyStruct {
//!     a: "a string".to_owned(),
//!     b: 32
//! };
//!
//! let mut de = Deserializer::new(StrRead::new(json));
//! inst.deserialize_over(&mut de)
//!     .expect("Failed to deserialize JSON");
//!
//! assert_eq!(inst.a, "test");
//! assert_eq!(inst.b, 32);
//! ```
//!
//! Here, the serialized json only has a value for the `a` field so when it gets
//! deserialized over the existing instance the `a` field is updated while the
//! `b` field remains unchanged.
//!
//! # Nested Structs
//! By default, the fields of the struct are deserialized using serde's
//! [`Deserialize`]. This means that, by default, nested structs must be
//! deserialized in entirety and cannot be deserialized on top of existing data.
//! To mark that subfields should instead be deserialized via
//! [`DeserializeOver`] the derive macro supports the `#[deserialize_over]`
//! attribute.
//!
//! ```
//! use serde_deserialize_over::DeserializeOver;
//! # use serde_json::Deserializer;
//! # use serde_json::de::StrRead;
//!
//! #[derive(DeserializeOver, Default)]
//! struct Inner {
//!     a: i32,
//!     b: i32
//! }
//!
//! #[derive(DeserializeOver, Default)]
//! struct Outer {
//!     #[deserialize_over]
//!     inner: Inner,
//!     c: i32
//! }
//!
//! let json = r#"{ "inner": { "b": 5 } }"#;
//! let mut inst = Outer::default();
//!
//! let mut de = Deserializer::new(StrRead::new(json));
//! inst.deserialize_over(&mut de)
//!     .expect("Failed to deserialize JSON");
//!
//! assert_eq!(inst.inner.a, 0);
//! assert_eq!(inst.inner.b, 5);
//! assert_eq!(inst.c, 0);
//! ```
//!
//! # Extras
//! This crate also provides the [`DeserializeInto`] extension trait on all
//! serde [`Deserializer`]s which takes the operands in the other order.
//!
//! ```
//! use serde_deserialize_over::{DeserializeOver, DeserializeInto};
//! # use serde_json::Deserializer;
//! # use serde_json::de::StrRead;
//!
//! #[derive(DeserializeOver, Default)]
//! struct MyStruct {
//!     pub a: String,
//!     pub b: i32
//! }
//!
//! let json = r#"{ "a": "test" }"#;
//! let mut inst = MyStruct::default();
//!
//! let mut de = Deserializer::new(StrRead::new(json));
//! de.deserialize_into(&mut inst)
//!   .expect("Failed to deserialize JSON");
//!
//! assert_eq!(inst.a, "test");
//! assert_eq!(inst.b, 0);
//! ```
//!
//! [`Deserialize`]: serde::Deserialize
//! [`Deserializer`]: serde::Deserializer

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

#[doc(hidden)]
pub use crate::support::DeserializeOverWrapper;
pub use serde_deserialize_over_derive::DeserializeOver;

use serde::Deserializer;

pub trait DeserializeOver<'de> {
  fn deserialize_over<D>(&mut self, de: D) -> Result<(), D::Error>
  where
    D: Deserializer<'de>;
}

pub trait DeserializeInto<'de>: Deserializer<'de> {
  fn deserialize_into<T>(self, target: &mut T) -> Result<(), Self::Error>
  where
    T: DeserializeOver<'de>,
  {
    target.deserialize_over(self)
  }
}

impl<'de, D> DeserializeInto<'de> for D where D: Deserializer<'de> {}
