use std::fs;

fn main() {
    let res = read_username_from_file();
    match res {
        Ok(username) => println!("{}", username),
        Err(err) => println!("{}", err),
    }
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    fs::read_to_string("username.txt")
}
