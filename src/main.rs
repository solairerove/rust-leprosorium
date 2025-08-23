fn main() {
    let x = 32;
    let _thirty_three = plus_one(Option::Some(x));
    plus_one(Option::None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
