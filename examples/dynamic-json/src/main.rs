use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    age: usize,
    email: String,
    full_name: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataV1 {
    plugins: PluginV1,
}

#[derive(Debug, Serialize, Deserialize)]
struct PluginV1 {
    width: Vec<Value>,
    height: Vec<Value>,
    z_index: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataV2 {
    plugins: HashMap<String, Value>,
}

fn main() -> Result<()> {
    // Typed deserialization
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

    // Type deserialization V1
    let data_v1 = r#"
    {
        "plugins": {
            "width": [["w", ["width"]]],
            "height": [["h", ["height"]]],
            "z_index": [["z", ["z-index"]]]
        }
    }
    "#;

    let d: DataV1 = serde_json::from_str(&data_v1)?;

    println!("Hi {:#?}", d);

    // Type deserialization V2
    let data_v2 = r#"
    {
        "plugins": {
            "width": [["w", ["width"]]],
            "height": [["h", ["height"]]],
            "z-index": [["z", ["z-index"]]]
        }
    }
    "#;

    let d: DataV2 = serde_json::from_str(&data_v2)?;
    let z_index: &Value = d.plugins.get("z-index").unwrap();
    let z_index_arr: &Vec<Value> = d.plugins.get("z-index").unwrap().as_array().unwrap();
    println!("{}", z_index);
    println!("{:?}", z_index_arr);

    Ok(())
}
