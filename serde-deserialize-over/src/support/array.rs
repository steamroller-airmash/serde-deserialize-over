use crate::{DeserializeOver, DeserializeOverWrapper};
use serde::{
    de::{SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

struct ArrayVisitor<'a, T, const N: usize>(&'a mut [T; N]);

impl<'de, 'a, T, const N: usize> Visitor<'de> for ArrayVisitor<'a, T, N>
where
    T: DeserializeOver<'de>,
{
    type Value = ();

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("an array of length {}", N))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        for item in self.0.iter_mut() {
            if let None = seq.next_element_seed(DeserializeOverWrapper(item))? {
                break;
            }
        }

        Ok(())
    }
}

impl<'de, T, const N: usize> DeserializeOver<'de> for [T; N]
where
    T: DeserializeOver<'de>,
{
    fn deserialize_over<D>(&mut self, de: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_tuple(self.len(), ArrayVisitor(self))
    }
}
