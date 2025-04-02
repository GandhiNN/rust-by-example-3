use serde::Serialize;

#[derive(Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn main() {
    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: Some(1234),
        keys: Keys {
            github: "xxxxxxxxxxxxxxx".to_string(),
            travis: Some("yyyyyyyyyyy".to_string()),
        },
    };

    let toml = toml::to_string(&config).unwrap();
    println!("{}", toml);
}
