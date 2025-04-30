use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn download(host: &str) -> Result<String, std::io::Error> {
    let target = (host, 80);
    let mut stream = TcpStream::connect(target).await?;

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;

    let mut content = Vec::new();
    stream.read_to_end(&mut content).await?;

    Ok(String::from_utf8(content).expect("UTF-8 conversion failed."))
}

#[tokio::main]
async fn main() {
    let download1 = tokio::spawn(download("example.com"));
    let download2 = tokio::spawn(download("example.net"));

    let result1 = download1
        .await
        .expect("First download crashed.")
        .expect("First download failed.");
    let result2 = download2
        .await
        .expect("Second download crashed.")
        .expect("Second download failed.");

    println!("{:?}, {:?}", result1, result2);
}
