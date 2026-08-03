use std::io::{self, BufRead};

fn parse_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
    // a.parse::<i32>() // placeholder
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();
    match parse_two(&a, &b) {
        Ok(n) => println!("sum: {}", n),
        Err(_) => println!("error: invalid input"),
    }
}
