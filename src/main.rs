fn main() {
    let mut s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;
    println!("{}, {}", r1, r2);
    // let r4 = r1;

    let r3 = &mut s1;
    println!("{}", r3);

    // println!("{}", r4);
}
