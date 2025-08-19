fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("The counter is {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
