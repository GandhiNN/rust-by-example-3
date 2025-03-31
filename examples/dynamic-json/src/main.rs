use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    age: usize,
    email: String,
    full_name: Vec<String>,
}

fn main() -> Result<()> {
    let data = r#"
    {
        "username": "ngakangandhi",
        "email": "ngakan.gandhi@gmail.com",
        "age": 31,
        "full_name": [
            "Ngakan",
            "Gandhi"
        ]
    }"#;

    let u: User = serde_json::from_str(&data)?;

    println!("Hi {}", u.full_name[0]);
    Ok(())
}
