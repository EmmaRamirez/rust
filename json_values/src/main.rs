// a lot of this is from https://github.com/whoisryosuke/rust-json-parser/tree/main
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

pub mod parser;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    vip: bool,
}

fn process_person() -> Result<()> {
    let data = r#"{"name": "John Doe", "age": 30, "vip": true}"#;
    let p: Person = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", p.name, p.age);
    Ok(())
}

fn create_person() -> Result<String> {
    let p = Person {
        name: "Emma".to_string(),
        age: 29,
        vip: true,
    };

    let j = serde_json::to_string(&p)?;
    Ok(j)
}

fn main() {
    process_person();
    create_person();

    let file_path = "data.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find file.");

    parser::typed_example(&contents);
    // println!("{}", j);

}