use serde_derive::Deserialize;
use serde_deserialize_over::DeserializeOver;
use std::time::Duration;

mod duration {
  use serde::{Deserialize, Deserializer};
  use std::time::Duration;

  pub(super) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Duration, D::Error> {
    f64::deserialize(de).map(Duration::from_secs_f64)
  }
}

#[derive(Deserialize, DeserializeOver)]
struct WithCustomSerialize {
  #[serde(with = "duration")]
  duration: Duration,
}

#[test]
fn works() {
  let json = r#"{ "duration": 50.0 }"#;
  let mut instance = WithCustomSerialize {
    duration: Duration::new(0, 0),
  };
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(json));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");

  assert_eq!(instance.duration, Duration::from_secs(50));
}
