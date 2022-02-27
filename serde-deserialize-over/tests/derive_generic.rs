use serde_deserialize_over::*;
use serde::{Serialize, Deserialize};

#[derive(DeserializeOver, Debug)]
struct Data<T> {
  value: T,
  other: i32
}
