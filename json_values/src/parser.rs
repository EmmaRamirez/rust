use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    vip: bool,
}

pub fn typed_example(contents: &String) -> Result<()> {
    let data = r#"
        {
            "name": "Emma",
            "age": 29,
            "vip": true
        }
    "#;

    let p: Person = serde_json::from_str(contents)?;

    println!("Please call {}", p.name);

    Ok(())
}