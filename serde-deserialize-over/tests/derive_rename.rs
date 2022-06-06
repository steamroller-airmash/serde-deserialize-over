use serde_deserialize_over::DeserializeOver;

#[derive(Default, DeserializeOver, Debug)]
struct ExampleStruct {
  #[serde(rename = "type")]
  pub a: String,
  pub b: i32,
}

#[test]
fn works() {
  let json = r#"{ "type": "test" }"#;
  let mut instance = ExampleStruct {
    a: "a string".to_owned(),
    b: 64,
  };
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(json));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");

  assert_eq!(instance.a, "test");
  assert_eq!(instance.b, 64);
}

#[test]
#[should_panic]
fn old_field_fails() {
  let json = r#"{ "a": "test" }"#;

  let mut instance = ExampleStruct {
    a: "a string".to_owned(),
    b: 64,
  };
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(json));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");
}
