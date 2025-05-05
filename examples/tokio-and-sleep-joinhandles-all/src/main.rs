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
    // Same as the for-loop in the earlier example
    // just more compact and functional
    let joinhandles = (0..10)
        .map(|i| tokio::spawn(async_task(i)))
        .collect::<Vec<_>>();

    println!("{} - main thread", now());

    futures::future::try_join_all(joinhandles).await.unwrap();
}
