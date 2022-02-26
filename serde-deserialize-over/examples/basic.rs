use serde_deserialize_over::DeserializeOver;

#[derive(Default, DeserializeOver, Debug)]
struct ExampleStruct {
  pub a: String,
  pub b: i32,
}

const JSON: &str = r#"{ "a": "test" }"#;

fn main() {
  let mut instance = ExampleStruct {
    a: "a string".to_owned(),
    b: 64,
  };
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(JSON));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");

  println!("{:#?}", instance);
}
