use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").expect("File not found");
}
