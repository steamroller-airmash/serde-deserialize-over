use serde::{Deserialize, Serialize};
use serde_deserialize_over::*;

#[derive(DeserializeOver, Debug)]
struct Data<T> {
  value: T,
  other: i32,
}
