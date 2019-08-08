use serde_deserialize_over::DeserializeOver;

#[derive(Default, DeserializeOver, Debug)]
struct StructA {
    #[deserialize_over]
    pub a: StructB,
    pub b: i32,
}

#[derive(Default, DeserializeOver, Debug)]
struct StructB {
    pub x: usize,
    pub y: String,
}

const JSON: &str = r#"{ "a": { "x": 1 }, "b": 0 }"#;

fn main() {
    let mut instance = StructA {
        a: StructB {
            x: 128,
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
