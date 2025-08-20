fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    // println!("{s2}");
    let s3 = takes_and_gives_back(s1);

    // println!("{s1}");,

    println!("{s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("give ownership");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
