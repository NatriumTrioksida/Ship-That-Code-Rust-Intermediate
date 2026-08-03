use std::io::{self, BufRead};

// fn append_excl(s: &mut String) { ... }

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let mut s = line.trim().to_string();
    // append_excl(&mut s);
    println!("{}", s);
}
