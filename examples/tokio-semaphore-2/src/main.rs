use futures::future::join_all;
use reqwest;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

async fn n_ops_at_a_time(n: i32) {
    for _ in 0..2 {
        // 20 times 50 = 100
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(async {
                let res = reqwest::get("http://www.rust-lang.org").await.unwrap();
                println!("Status: {}", res.status());
            });
        }
        join_all(v).await;
    }
}

async fn n_ops_at_a_time_semaphore(n: usize) {
    let mut v = Vec::new();
    let permits = Arc::new(Semaphore::new(n));
    for _ in 0..100 {
        let permits = permits.clone();
        v.push(async move {
            let _permit = permits.acquire_owned().await;
            let res = reqwest::get("https://www.rust-lang.org").await.unwrap();
            println!("Status: {}", res.status());
        });
    }
    join_all(v).await;
}

#[tokio::main]
async fn main() {
    let now = Instant::now();
    n_ops_at_a_time(50).await;
    let elapsed = now.elapsed();
    println!("Elapsed without semaphore: {:.2?}", elapsed);

    let now = Instant::now();
    n_ops_at_a_time_semaphore(50).await;
    let elapsed = now.elapsed();
    println!("Elapsed Semaphore: {:.2?}", elapsed);
}
