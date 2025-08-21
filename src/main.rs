fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black[0] {}", black.0);
    println!("origin[2] {}", origin.2);

    let Point(x, y, z) = origin;
    println!("x: {}, y: {}, z: {}", x, y, z);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
