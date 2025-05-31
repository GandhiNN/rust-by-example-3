use futures::future::join_all;
use tokio::time::Duration;

async fn task_one() -> u32 {
    // Simulate some time-consuming operation
    tokio::time::sleep(Duration::from_secs(2)).await;
    42
}

async fn task_two() -> u32 {
    // Simulate another async operation
    tokio::time::sleep(Duration::from_secs(1)).await;
    24
}

async fn fetch_data_from_url(url: &str) -> String {
    // Simulate fetching data from a remote server
    tokio::time::sleep(Duration::from_secs(2)).await;
    format!("Data from {}", url)
}

#[tokio::main]
async fn main() {
    let (result_one, result_two) = tokio::join!(task_one(), task_two());
    println!("Result of task one: {}", result_one);
    println!("Result of task two: {}", result_two);

    let urls = vec![
        "https://example.com",
        "https://rust-lang.org",
        "https://github.com",
    ];
    let mut results = vec![];

    // Sequential approach:
    // We iterate over the URLs in a loop and await each
    // `fetch_data_from_url` call sequentially.
    // This means that the requests are made one after the other, and
    // we wait for each one to complete before starting the next
    for url in &urls {
        let result = fetch_data_from_url(url).await;
        results.push(result);
    }

    // Parallel approach with join_all:
    // We use `join_all` to create a vector of futures
    // for each URL and then await them all concurrently.
    // This allows all the requests to be initiated simultaneously,
    // significantly reducing the total execution time.
    let futures = urls.iter().map(|url| fetch_data_from_url(url));
    let parallel_results = join_all(futures).await;
    println!("Sequential results: {:#?}", results);
    println!("Parallel results: {:#?}", parallel_results);
}
