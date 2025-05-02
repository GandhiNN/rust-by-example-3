use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

async fn task_one(sender: mpsc::Sender<String>, mut receiver: mpsc::Receiver<String>) {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                sender.send("Rabbit Running".to_string()).await.expect("Failed to send message");
            }
            msg = receiver.recv() => {
                if let Some(msg) = msg {
                    println!("Task 1 received: {}", msg);
                }
            }
        }
    }
}

async fn task_two(sender: mpsc::Sender<String>) {
    let mut interval = time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        sender
            .send("Turtle Running".to_string())
            .await
            .expect("Failed to send message")
    }
}

#[tokio::main]
async fn main() {
    let (task1_sender, mut task1_receiver) = mpsc::channel::<String>(10);
    let (main_sender_to_task1, main_receiver_of_task1) = mpsc::channel::<String>(10);
    let (task2_sender, mut task2_receiver) = mpsc::channel::<String>(10);

    let _task1 = tokio::spawn(task_one(task1_sender.clone(), main_receiver_of_task1));
    let _task2 = tokio::spawn(task_two(task2_sender.clone()));

    tokio::spawn(async move {
        loop {
            tokio::select! {
                result = task1_receiver.recv() => {
                    match result {
                        Some(msg) => {
                            println!("Received from Task 1: {}", msg);
                        }
                        None => {
                            println!("Task 1 channel closed");
                            // Handle the channel being closed if necessary
                        }
                    }
                }
                result = task2_receiver.recv() => {
                    match result {
                        Some(msg) => {
                            println!("Received from Task 2: {}", msg);
                        }
                        None => {
                            println!("Task 2 channel closed");
                            // Handle the channel being closed if necessary
                        }
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        for i in 0..5 {
            main_sender_to_task1
                .send(format!("Message Talk to Press {}", i))
                .await
                .expect("Failed to send to task1");
            // Sending messages from the main task to task_one
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    // exit code begins
    let ctrl_c = signal::ctrl_c();
    println!("Press Ctrl+C to exit...");

    // Wait for the Ctrl+C signal
    ctrl_c.await.expect("Ctrl+C signal failed");

    println!("Ctrl+C received! Exiting...");
}
