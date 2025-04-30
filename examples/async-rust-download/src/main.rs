// use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = set_client();
    // let resp = client
    //     .get("https://link.testfile.org/500MB")
    //     .send()
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    let resp = client.get("https://link.testfile.org/500MB").send().await?;
    println!("{:#?}", resp);
    Ok(())
}

pub fn set_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
}
