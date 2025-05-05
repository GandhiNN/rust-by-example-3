use futures::future::join_all;
use std::thread;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let future_1 = async_operation(1);
    let future_2 = async_operation(2);
    join_all([future_1, future_2]).await;

    println!("{}: {:?}", "futures: ", start.elapsed());
}

async fn async_operation(thread: i8) {
    for i in 1..5 {
        println!("{} from the operation {}!", i, thread);
        // simulates some IO wait
        tokio::time::sleep(Duration::from_millis(400)).await;
        // simulates some CPU workload
        thread::sleep(Duration::from_millis(100));
    }
}
