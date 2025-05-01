use futures::future::join_all;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://example.com/1.html".to_string(),
        "https://example.com/2.html".to_string(),
        "https://example.com/3.html".to_string(),
    ];

    // Iterate over the paths.
    let mut tasks: Vec<JoinHandle<Result<(), ()>>> = vec![];

    for url in urls {
        // Copy each path into a new string
        // that can be consumed/captured by the task closure
        let url_copy = url.clone();

        // Create a Tokio task for each path
        tasks.push(tokio::spawn(async move {
            match reqwest::get(&url_copy).await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        println!("RESPONSE: {} bytes from {}", text.len(), url_copy);
                    }
                    Err(_) => {
                        println!("ERROR reading {}", url_copy)
                    }
                },
                Err(_) => {
                    println!("ERROR downloading {}", url_copy);
                }
            }
            Ok(())
        }));
    }

    // Wait for them all to finish
    println!("Started {} tasks. Waiting...", tasks.len());
    join_all(tasks).await;
}
