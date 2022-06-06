use serde_deserialize_over::DeserializeOver;

#[derive(Default, DeserializeOver, Debug)]
struct ExampleStruct {
  #[serde(rename = "type")]
  pub a: String,
  pub b: i32,
}

const JSON: &str = r#"{ "type": "test" }"#;

#[test]
fn works() {
  let mut instance = ExampleStruct {
    a: "a string".to_owned(),
    b: 64,
  };
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(JSON));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");

  assert_eq!(instance.a, "test");
  assert_eq!(instance.b, 64);
}
