use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    is_student: bool,
}

fn main() -> Result<()> {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        is_student: false,
    };

    // Serialize it to a JSON string.
    let serialized = serde_json::to_string(&person)?;
    println!("Serialized: {}", serialized);

    // Deserialize it back to a Rust struct.
    let deserialized: Person = serde_json::from_str(&serialized)?;
    println!("Deserialized: {:?}", deserialized);

    Ok(())
}
