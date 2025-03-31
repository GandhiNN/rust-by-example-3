use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use serde::Deserialize;

const CLIENT_ID: &str = "8dcf83db60d94d638572e591695c5945";
const CLIENT_SECRET: &str = "3eb6ceb5b8674ed5934075e7c55fa88d";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Token {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

async fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    let auth = format!(
        "Basic {}",
        BASE64.encode(format!("{}:{}", CLIENT_ID, CLIENT_SECRET))
    );
    let client = reqwest::Client::new();
    let params = [("grant_type", "client_credentials")];
    let body = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", &auth)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    // Get access token
    let tok_resp = get_token().await;
    let auth_token = match tok_resp {
        Ok(x) => {
            let token: Token = serde_json::from_str(&x).unwrap();
            token.access_token
        }
        Err(_) => String::from("token error"),
    };

    println!("{}", auth_token);

    // Get the data
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = "Megadeth"
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("{:#?}", response.json::<serde_json::Value>().await);
        }
        _ => {
            panic!("Something unexpected happened");
        }
    }
}
