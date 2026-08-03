use std::io::Read;

fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    // return a closure that increments and returns count
    move || {
        count += 1; // placeholder
        count
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.split_whitespace().next().unwrap().parse().unwrap();
    let mut c = make_counter();
    for _ in 0..n {
        println!("{}", c());
    }
}
