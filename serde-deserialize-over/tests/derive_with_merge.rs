use serde_derive::Deserialize;
use serde_deserialize_over::DeserializeOver;

mod custom {
  use super::DeserializeOver;
  use super::Inner;
  use serde::Deserializer;

  pub(super) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Inner, D::Error> {
    let mut inner = Inner::default();
    inner.deserialize_over(de)?;
    Ok(inner)
  }

  pub(super) fn deserialize_over<'de, D: Deserializer<'de>>(
    de: D,
    inner: &mut Inner,
  ) -> Result<(), D::Error> {
    inner.b = "bar".to_owned();
    inner.deserialize_over(de)
  }
}

#[derive(Default, DeserializeOver)]
struct Inner {
  a: String,
  b: String,
}

#[derive(Deserialize, DeserializeOver)]
struct WithCustomSerialize {
  #[deserialize_over]
  #[serde(with = "custom")]
  inner: Inner,
}

#[test]
fn works() {
  let json = r#"{ "inner": { "a": "test" } }"#;
  let mut instance = WithCustomSerialize {
    inner: Inner {
      a: "blah".to_owned(),
      b: "foo".to_owned(),
    },
  };
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(json));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");

  assert_eq!(instance.inner.a, "test");
  assert_eq!(instance.inner.b, "bar");
}
