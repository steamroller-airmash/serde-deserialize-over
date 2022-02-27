use crate::{DeserializeOver, DeserializeOverWrapper};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::{
  collections::{hash_map::Entry, HashMap},
  fmt,
  hash::{BuildHasher, Hash},
};

struct MapVisitor<'a, K, V, S>(&'a mut HashMap<K, V, S>);

impl<'de, 'a, K, V, S> Visitor<'de> for MapVisitor<'a, K, V, S>
where
  K: Deserialize<'de> + Eq + Hash,
  V: Deserialize<'de> + DeserializeOver<'de>,
  S: BuildHasher,
{
  type Value = ();

  fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("a map")
  }

  fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
  where
    A: MapAccess<'de>,
  {
    while let Some(key) = map.next_key()? {
      match self.0.entry(key) {
        Entry::Occupied(mut entry) => {
          map.next_value_seed(DeserializeOverWrapper(entry.get_mut()))?
        }
        Entry::Vacant(entry) => {
          entry.insert(map.next_value()?);
        }
      }
    }

    Ok(())
  }
}

impl<'de, K, V, S> DeserializeOver<'de> for HashMap<K, V, S>
where
  K: Deserialize<'de> + Eq + Hash,
  V: Deserialize<'de> + DeserializeOver<'de>,
  S: BuildHasher,
{
  fn deserialize_over<D>(&mut self, de: D) -> Result<(), D::Error>
  where
    D: Deserializer<'de>,
  {
    de.deserialize_map(MapVisitor(self))
  }
}
