use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().expect("error joining");
}
