#[doc(hidden)]
pub mod export {
    pub use serde::de::{Error, MapAccess, SeqAccess, Unexpected, Visitor};
    pub use serde::{Deserialize, Deserializer};

    pub use std::fmt;
    pub use std::marker::PhantomData;
    pub use std::option::Option::{None, Some};
    pub use std::result::Result::{self, Err, Ok};
}

use serde::{de::DeserializeSeed, Deserialize, Deserializer};
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

impl<T> DeserializeOver for Option<T>
where
    T: DeserializeOver + for<'d> Deserialize<'d>,
{
    fn deserialize_over<'de, D>(&mut self, de: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Visitor;
        use std::fmt;

        struct OptionVisitor<'a, U>(&'a mut Option<U>);

        impl<'a, 'de, U> Visitor<'de> for OptionVisitor<'a, U>
        where
            U: DeserializeOver + Deserialize<'de>,
        {
            type Value = ();

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "an option")
            }

            fn visit_some<D>(self, de: D) -> Result<(), D::Error>
            where
                D: Deserializer<'de>,
            {
                match self.0 {
                    Some(x) => x.deserialize_over(de)?,
                    None => *self.0 = Some(Deserialize::deserialize(de)?),
                }

                Ok(())
            }

            fn visit_none<E>(self) -> Result<(), E> {
                Ok(())
            }
        }

        de.deserialize_option(OptionVisitor(self))
    }
}
