use std::fmt::Display;

fn main() {
    let point = Pair { x: 12, y: 21 };

    let another_one = Pair {
        x: String::from("x"),
        y: String::from("y"),
    };

    point.cmp_display();
    another_one.cmp_display()
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
