use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    // Serialize from struct
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

    // Deserialize from json string
    let data_invalid = r#"
    {
        "name": "John Doe",
        "age": "twenty",
        "email": "john@example.com"
    }
    "#;

    let result: Result<User, _> = serde_json::from_str(&data_invalid);

    match result {
        Ok(_) => println!("Successfully deserialized data"),
        Err(err) => {
            println!("We ran into an error: {}", err);
            match err.classify() {
                serde_json::error::Category::Io => println!("Problem reading file"),
                serde_json::error::Category::Syntax => println!("Problem with JSON syntax"),
                serde_json::error::Category::Data => println!("Problem with data"),
                serde_json::error::Category::Eof => println!("Unexpected end of file"),
            }
        }
    }

    let data_valid = r#"
    {
        "name": "Gandhi Ngakan",
        "age": 31,
        "email": "ngakan.gandhi@sampoerna.com"
    }
    "#;

    let result: Result<User, _> = serde_json::from_str(&data_valid);

    match result {
        Ok(res) => println!("Successfully deserialized data: {:#?}", res),
        Err(err) => {
            println!("We ran into an error: {}", err);
            match err.classify() {
                serde_json::error::Category::Io => println!("Problem reading file"),
                serde_json::error::Category::Syntax => println!("Problem with JSON syntax"),
                serde_json::error::Category::Data => println!("Problem with data"),
                serde_json::error::Category::Eof => println!("Unexpected end of file"),
            }
        }
    }
}
