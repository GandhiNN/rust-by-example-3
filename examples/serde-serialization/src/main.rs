use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    let user = User {
        name: String::from("Sarah"),
        age: 32,
        email: String::from("sarah@example.com"),
    };

    // Serialize
    let serialized = serde_json::to_string(&user).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize
    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
