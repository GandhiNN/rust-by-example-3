use chrono::Local;
use std::thread;
use std::time::{Duration, Instant};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn async_task(i: u64) {
    thread::sleep(Duration::from_secs(i));
    println!("{} - async task {}!", now(), i);
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    for i in 0..10 {
        tokio::spawn(async_task(i));
    }
    println!("{} - main thread", now());
    println!("{}: {:?}", "elapsed: ", start.elapsed());
}
