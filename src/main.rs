fn main() {
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 5.0, y: 6.0 };

    println!("Point is at ({}, {})", integer.x, integer.y);
    println!("Point is at ({}, {})", float.x, float.y);
}

struct Point<T> {
    x: T,
    y: T,
}
