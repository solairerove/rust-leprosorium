fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum number is {}", max);
    }
}
