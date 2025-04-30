use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let target = ("example.net", 80);
    let mut stream = TcpStream::connect(target)
        .await
        .expect("Connection failed.");

    stream
        .write_all(b"GET / HTTP/1.0\r\n\r\n")
        .await
        .expect("Write failed.");

    let mut content = Vec::new();
    stream
        .read_to_end(&mut content)
        .await
        .expect("Read failed.");

    let text = String::from_utf8(content).expect("UTF-8 conversion failed.");
    println!("{:?}", text);
}
