# serde-deserialize-over

This library provides a trait + derive macro that allows you
to deserialize partial values on top of an existing struct
instance.

# Examples

```rust
use serde_deserialize_over::DeserializeOver;

#[derive(Default, Debug, DeserializeOver)]
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

```

By default, struct members are deserialized using the `serde::Deserialize`
trait. However, if you want to continue deserializing over members then
you can enforce that by marking them with the `#[deserialize_over]` 
attribute.

```rust
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
```
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
