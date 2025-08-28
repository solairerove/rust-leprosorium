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
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
