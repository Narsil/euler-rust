use crate::eleven_twenty::triangle;
use std::fs::read_to_string;

pub fn pb67() {
    let data = read_to_string("data/p067_triangle.txt").unwrap();
    println!("Pb67 {}", triangle(&data));
}
