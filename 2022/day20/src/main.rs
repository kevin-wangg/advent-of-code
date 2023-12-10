use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut a: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
}
