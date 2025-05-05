use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });

    handle.join().expect("error joining");
}
