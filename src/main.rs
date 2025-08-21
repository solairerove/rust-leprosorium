fn main() {
    let s1 = String::from("hello");

    change(&s1);

    println!("{}", s1);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
