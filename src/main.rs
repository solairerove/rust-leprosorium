fn main() {
    let mut s1 = String::from("hello");

    let r1 = &mut s1;
    let r2 = &mut s1;

    println!("{}, {}", r1, r2);
}
