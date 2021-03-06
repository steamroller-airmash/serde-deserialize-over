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

const JSON: &str = r#"{ "a": "test" }"#;

fn main() {
  let mut instance = ExampleStruct::<String> {
    a: "a string".to_owned(),
    b: 64,
  };
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(JSON));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");

  println!("{:#?}", instance);
}
