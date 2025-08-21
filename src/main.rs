fn main() {
    let mut user = User {
        active: true,
        username: String::from("solairerove"),
        email: String::from("solairerove@gmail.com"),
        sign_in_count: 1,
    };

    println!("{:?}", user.sign_in_count);

    user.sign_in_count = user.sign_in_count + 1;

    println!("{:?}", user.sign_in_count);

    let user1 = build_user(String::from("pizda@gmail.com"), String::from("hui"));

    println!("email: {} username: {}", user1.email, user1.username);

    let user2 = User {
        active: false,
        username: user1.username,
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };

    println!("{:?}", user2.active);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
