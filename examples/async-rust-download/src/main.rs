#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/ip";
    let client = set_client(true);
    match client.get(url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                println!("RESPONSE: {} bytes from {}", text.len(), url);
            }
            Err(_) => {
                println!("ERROR reading {}", url)
            }
        },
        Err(_) => {
            println!("ERROR downloading {}", url)
        }
    }
    Ok(())
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
