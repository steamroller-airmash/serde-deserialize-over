use serde_deserialize_over::DeserializeOver;

#[derive(DeserializeOver, Debug)]
struct StructA {
  #[deserialize_over]
  pub a: StructB,
  pub b: i32,
}

#[derive(DeserializeOver, Debug)]
struct StructB {
  #[deserialize_over]
  pub x: StructC,
  pub y: String,
}

#[derive(DeserializeOver, Debug)]
struct StructC {
  pub x: usize,
  pub y: String,
}

const JSON: &str = r#"{ "a": { "x": { "x": 1 } }, "b": 0 }"#;

fn main() {
  let mut instance = StructA {
    a: StructB {
      x: StructC {
        x: 2,
        y: "another string".to_owned(),
      },
      y: "a string".to_owned(),
    },
    b: 64,
  };
  let mut de = serde_json::Deserializer::new(serde_json::de::StrRead::new(JSON));

  instance
    .deserialize_over(&mut de)
    .expect("Failed to deserialize");

  println!("{:#?}", instance);
}
