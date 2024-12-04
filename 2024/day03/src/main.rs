use std::env;
use std::fs;

use regex::Regex;

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do(n't)?\(\)").unwrap();
    let mut ans = 0;
    for group in re.captures_iter(input) {
        let x = group.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = group.get(2).unwrap().as_str().parse::<i32>().unwrap();
        ans += x * y;
    }
    println!("ans: {ans}");
}

fn part2(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(don't\(\))|(do\(\))").unwrap();
    let mut ans = 0;
    let mut enabled = true;
    for group in re.captures_iter(input) {
        let cap = group.get(0).unwrap().as_str();
        if cap == "do()" {
            enabled = true;
        } else if cap == "don't()" {
            enabled = false;
        } else {
            let x = group.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = group.get(2).unwrap().as_str().parse::<i32>().unwrap();
            if enabled {
                ans += x * y;
            }
        }
    }
    println!("ans: {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    part1(&input);
    part2(&input);
}
