use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use reqwest::{self};
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
    // let client = reqwest::Client::new();
    // let response = client
    //     .get("https://api.spotify.com/v1/search")
    //     .header(AUTHORIZATION, "Bearer BQDCsPOOXDCKYFuj1TbMJAuLwG7_ei5je_KsGC_36-I6qE_r7i4bgHWqOiL6jEIhsuncAMTz5ArRBVy7OcgigMvouH2cP-jKVSfWanFUMbyRlp68szpy57G_nYOXOLsXfNWX_KMNhU8")
    //     .header(CONTENT_TYPE, "application/json")
    //     .header(ACCEPT, "application/json")
    //     .send()
    //     .await
    //     .unwrap()
    //     .text()
    //     .await;

    // println!("Success! {:#?}", response);
    let resp = get_token().await;
    match resp {
        Ok(x) => {
            let token: Token = serde_json::from_str(&x).unwrap();
            let bearer_token = token.access_token;
            println!("{}", bearer_token);
        }
        Err(e) => eprintln!("{}", e),
    };
}
