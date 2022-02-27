use serde_deserialize_over::DeserializeOver;

#[derive(Default, DeserializeOver, Debug)]
struct ExampleStruct<T> {
  pub a: T,
  pub b: i32,
}

#[derive(DeserializeOver)]
struct WithConstraints<T: Default> {
  pub a: T,
}

#[derive(DeserializeOver)]
struct WithConstGenerics<const N: usize> {
  #[deserialize_over]
  pub a: [ExampleStruct<()>; N],
}

const JSON: &str = r#"{ "a": "test" }"#;

#[test]
fn works() {
  let mut instance = ExampleStruct::<String> {
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
