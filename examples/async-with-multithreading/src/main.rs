use std::{thread, time::Duration};
use tokio;

#[tokio::main]
async fn main() {
    let spawn_1 = tokio::spawn(async {
        for i in 1..5 {
            println!("{} from the thread 1!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let spawn_2 = tokio::spawn(async {
        for i in 1..5 {
            println!("{} from the thread 2!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    spawn_1.await.expect("error awaiting");
    spawn_2.await.expect("error awaiting");
}
