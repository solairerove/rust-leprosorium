fn main() {
    let mut counter = 0;

    'counting_loop: loop {
        println!("Counter: {}", counter);
        let mut remaining = 10;

        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }

            if counter == 2 {
                break 'counting_loop;
            }

            remaining -= 1;
        }

        counter += 1;
    }

    println!("End of counter: {}", counter);
}
