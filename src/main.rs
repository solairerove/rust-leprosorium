fn main() {
    let x = plus_one(4);

    println!("{}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
