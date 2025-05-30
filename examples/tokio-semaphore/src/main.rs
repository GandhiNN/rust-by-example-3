use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

#[tokio::main]
async fn main() {
    // Define the maximum number of concurrent connections
    let max_concurrent_connections = 5;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_connections));

    // Simulate a server handling multiple connections
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let sem_clone = Arc::clone(&semaphore);
            task::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                // Simulating connection handling
                println!("Handling connection {}", i);
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                println!("Finished connection {}", i);
            })
        })
        .collect();

    // Await all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}
