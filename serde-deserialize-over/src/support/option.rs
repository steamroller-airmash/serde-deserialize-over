use crate::DeserializeOver;
use serde::{de::Visitor, Deserialize, Deserializer};
use std::fmt;

struct OptionVisitor<'a, U>(&'a mut Option<U>);

impl<'a, 'de, U> Visitor<'de> for OptionVisitor<'a, U>
where
    U: DeserializeOver<'de> + Deserialize<'de>,
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

impl<'de, T> DeserializeOver<'de> for Option<T>
where
    T: DeserializeOver<'de> + Deserialize<'de>,
{
    fn deserialize_over<D>(&mut self, de: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_option(OptionVisitor(self))
    }
}
