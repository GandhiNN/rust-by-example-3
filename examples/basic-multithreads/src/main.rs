use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..100 {
            println!("{} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
