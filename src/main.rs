use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).cloned().unwrap_or(0);
    println!("{:?}", score);
}
