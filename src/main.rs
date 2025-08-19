fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut idx = 0;

    while idx < a.len() {
        println!("a[{}] = {}", idx, a[idx]);
        idx += 1;
    }
}
