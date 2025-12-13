use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();
    let input = input.trim();
    let part = env::args().nth(2).unwrap().parse::<i32>().unwrap();
    if part == 1 {
        solve1(&input);
    } else {
        solve2(&input);
    }
}

fn solve1(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut max = 0;
        let mut index = 0;
        for i in 0..digits.len() - 1 {
            if digits[i] > max {
                max = digits[i];
                index = i;
            }
        }
        let mut second_max = 0;
        for i in index + 1..digits.len() {
            if digits[i] > second_max {
                second_max = digits[i];
            }
        }
        let joltage = max * 10 + second_max;
        ans += joltage;

    }
    println!("ans {ans}");
}

fn max_joltage(digits: &[u32], n: u32) -> String {
    if n == 0 {
        return String::new();
    }
    assert!(digits.len() >= n as usize);
    let mut max = 0;
    let mut index = 0;
    for i in 0..digits.len() - n as usize + 1 {
        if digits[i] > max {
            max = digits[i];
            index = i;
        }
    }
    let rest = max_joltage(&digits[index + 1..], n - 1);         
    return max.to_string() + &rest;
}

fn solve2(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let digits = line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let joltage = max_joltage(&digits, 12).parse::<u64>().unwrap();
        ans += joltage;
    }
    println!("ans {ans}");
}
