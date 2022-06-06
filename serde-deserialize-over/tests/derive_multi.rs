use serde_deserialize_over::*;

#[derive(DeserializeOver, Default)]
struct A {
  b: String,
}

#[derive(DeserializeOver, Default)]
struct B {
  #[deserialize_over]
  #[serde(rename = "test")]
  a: A,
}

#[test]
fn works() {
  let json = r#"{ "test": { "b": "AAAAA" } }"#;
  let mut instance = B::default();
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(json));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");

  assert_eq!(instance.a.b, "AAAAA");
}
