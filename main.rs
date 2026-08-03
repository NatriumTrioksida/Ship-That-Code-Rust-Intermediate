use std::io::{self, BufRead};

trait Shape {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let kind = lines.next().unwrap().unwrap();
    let dim: f64 = lines.next().unwrap().unwrap().parse().unwrap();
    let s: Box<dyn Shape> = if kind == "circle" {
        Box::new(Circle { radius: dim })
    } else {
        Box::new(Square { side: dim })
    };
    println!("{:.2}", s.area());
}
