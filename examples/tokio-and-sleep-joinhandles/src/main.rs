use chrono::Local;
use std::time::Duration;

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn async_task(i: u64) {
    tokio::time::sleep(Duration::from_secs(i)).await;
    println!("{} - async task {}!", now(), i);
}

#[tokio::main]
async fn main() {
    let mut joinhandles = Vec::new();
    for i in 0..10 {
        joinhandles.push(tokio::spawn(async_task(i)));
    }
    println!("{} - main thread", now());

    for joinhandle in joinhandles {
        joinhandle.await.unwrap();
    }
}
