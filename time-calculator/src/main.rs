extern crate rust_kata_time;

use rust_kata_time::time;

fn main() {
    let first = time::TimeInterval::parse("1m 45s").unwrap();
    let second = time::TimeInterval::parse("1h 19s").unwrap();
    let sum = first.add(&second);
    println!("{}", sum.as_string());
}
