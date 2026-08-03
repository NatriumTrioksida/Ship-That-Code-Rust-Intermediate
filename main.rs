use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let total: i32 = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("{}", total);
}
