use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, json};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str which might come
    // from the user
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }
    "#;

    // Parse the string of data into serde_json::Value
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str which might
    // comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }
    "#;

    // Parse the string of data into a Person object.
    // This is exactly the same function as the one
    // that produced serde_json::Value above, but now
    // we are asking it for a Person as output
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

fn construct_json() {
    // the type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}

fn main() {
    let _ = untyped_example();
    let _ = typed_example();
    construct_json();
}
