enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
}
