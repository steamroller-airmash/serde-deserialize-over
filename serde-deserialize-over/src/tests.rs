//! This module abuses doctests to create tests that are supposed to fail.

/// ```compile_fail
/// use serde_deserialize_over::*;
/// use serde::*;
///
/// fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<(), D::Error> {
///   Ok(())
/// }
///
/// #[derive(DeserializeOver)]
/// struct BadDeserialize {
///   #[deserialize_over]
///   #[serde(deserialize_with = "deserialize")]
///   field: ()
/// }
/// ```
mod combo_deserialize_with_and_deserialize_over {}

/// ```compile_fail
/// use serde_deserialize_over::*;
/// use serde::*;
/// 
/// #[derive(DeserializeOver)]
/// struct MultiRename {
///   #[serde(rename = "a", deserialize = "b")]
///   field: ()
/// }
/// ```
mod combo_rename_and_deserialize {}

/// ```compile_fail
/// use serde_deserialize_over::*;
/// use serde::*;
/// 
/// #[derive(DeserializeOver)]
/// struct RepeatedOption {
///   #[serde(rename = "x", rename = "y")]
///   field: ()
/// }
/// ```
mod duplicate_option {}
