use std::{thread, time::Duration};

fn main() {
    let handle_1 = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from the spawned thread 1!", i);
            thread::sleep(Duration::from_millis(1));
        }
        return 100;
    });

    let handle_2 = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from the spawned thread 2!", i);
            thread::sleep(Duration::from_millis(1));
        }
        return 200;
    });

    let result_1 = handle_1.join().expect("error joining");
    let result_2 = handle_2.join().expect("error joining");

    println!("final result: {} from the main thread", result_1 + result_2);
}
