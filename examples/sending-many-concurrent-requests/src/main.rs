use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use futures::StreamExt;
use futures::stream;

fn read_lines(config: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(config)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}

pub fn set_client(allow_self_signed_cert: bool) -> reqwest::Client {
    if allow_self_signed_cert {
        reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
    } else {
        reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(false)
            .build()
            .unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let paths: Vec<String> = read_lines(
        "/Users/ngakangandhi/study/rust/rust-by-example-3/examples/sending-many-concurrent-requests/urls.txt",
    )?;
    let client = set_client(true);
    let fetches = stream::iter(paths.into_iter().map(|path| {
        let send_fut = client.get(&path).send();
        async move {
            match send_fut.await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        println!("RESPONSE: {} bytes from {}", text.len(), path);
                    }
                    Err(_) => println!("ERROR reading {}", path),
                },
                Err(_) => println!("ERROR downloading {}", path),
            }
        }
    }))
    .buffer_unordered(100)
    .collect::<Vec<()>>();
    fetches.await;
    Ok(())
}
