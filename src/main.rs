use std::fs::File;
use std::io::Read;

fn main() {
    let res = read_username_from_file();
    match res {
        Ok(username) => println!("{}", username),
        Err(err) => println!("{}", err),
    }
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
