fn main() {
    let mut x = 0;
    let mut y = 1;
    let nth_fibonacci_number = 6;

    for i in 0..nth_fibonacci_number {
        let temp = x;
        x = y;
        y = y + temp;
    }

    print!("{y}");
}
// 0 1 1 2 3 5 8 13 21