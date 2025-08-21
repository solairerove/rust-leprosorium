fn main() {
    let user = User {
        active: true,
        username: String::from("solairerove"),
        email: String::from("solairerove@gmail.com"),
        sign_in_count: 1,
    };

    println!("{:?}", user.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
