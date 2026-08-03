use std::io::{self, BufRead};

mod geometry {
    pub fn circle_area(dim: f64) -> f64 {
        3.14 * dim * dim
    }

    pub fn square_area(dim: f64) -> f64 {
        dim * dim
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let kind: String = lines.next().unwrap().unwrap();
    let dim: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let area = if kind.trim() == "circle" {
        geometry::circle_area(dim)
        // 0.0
    } else {
        geometry::square_area(dim)
        // 0.0
    };
    println!("{:.2}", area);
}
