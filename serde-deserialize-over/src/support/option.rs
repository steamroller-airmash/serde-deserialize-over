use crate::DeserializeOver;
use serde::{de::Visitor, Deserialize, Deserializer};
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

impl<T> DeserializeOver for Option<T>
where
    T: DeserializeOver + for<'d> Deserialize<'d>,
{
    fn deserialize_over<'de, D>(&mut self, de: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_option(OptionVisitor(self))
    }
}
