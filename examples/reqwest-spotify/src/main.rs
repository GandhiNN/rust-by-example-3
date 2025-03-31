use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = "8dcf83db60d94d638572e591695c5945";
const CLIENT_SECRET: &str = "3eb6ceb5b8674ed5934075e7c55fa88d";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AccessToken {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("name: {}", track.name);
        println!("album: {}", track.album.name);
        println!(
            "artists: {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("urls: {}", track.external_urls.spotify);
    }
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
            let token: AccessToken = serde_json::from_str(&x).unwrap();
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

    // println!("{:#?}", response.json::<serde_json::Value>().await);

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("The response did not match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Something unexpected happened {:?}", other);
        }
    }
}
