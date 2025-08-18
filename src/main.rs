fn main() {
    let tup: (i32, f64, u8) = (500, 6.5, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_five = tup.1;
    let one = tup.2;

    println!("[0]: {five_hundred}, [1]: {six_point_five}, [2]: {one}")
}
