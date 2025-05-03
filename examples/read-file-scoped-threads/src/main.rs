use std::fs;
use std::thread;
use std::time::Instant;

const CUR_DIR: &str =
    "/Users/ngakangandhi/study/rust/rust-by-example-3/examples/read-file-scoped-threads/";

fn scoped_read() {
    let files = [
        format!("{CUR_DIR}/foo.txt"),
        format!("{CUR_DIR}/bar.txt"),
        format!("{CUR_DIR}/baz.txt"),
    ];
    thread::scope(|scope| {
        for file in files {
            scope.spawn(move || {
                let contents = fs::read_to_string(file);
                println!("{:#?}", contents);
            });
        }
    })
}

fn standard_read() {
    let files = [
        format!("{CUR_DIR}/foo.txt"),
        format!("{CUR_DIR}/bar.txt"),
        format!("{CUR_DIR}/baz.txt"),
    ];
    for file in files {
        let contents = fs::read_to_string(file);
        println!("{:#?}", contents);
    }
}

fn main() {
    let start = Instant::now();
    standard_read();
    let duration = start.elapsed();
    println!("Time elapsed for standard_read() = {:#?}", duration);

    let start = Instant::now();
    scoped_read();
    let duration = start.elapsed();
    println!("Time elapsed for scoped_read() = {:#?}", duration);
}
