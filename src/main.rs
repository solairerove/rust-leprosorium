use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("{:?}", e),
        },
        _ => {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("{}", greeting_file.metadata().unwrap().len());
}
