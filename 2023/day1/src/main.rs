use std::collections::HashMap;
use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let path = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let input = fs::read_to_string(path).expect("File should exist");
    let mut ans = 0;
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9").unwrap();
    let rev_re = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|1|2|3|4|5|6|7|8|9").unwrap();
    let map = HashMap::from(
        [
            ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"),
            ("seven", "7"), ("eight", "8"), ("nine", "9"),
        ]
    );
    for line in input.lines() {
        let first_digit = re.find(line).unwrap().as_str();
        let first_digit = match first_digit.parse::<i32>() {
            Ok(_) => first_digit,
            Err(_) => map[first_digit]
        };
        let rev: String = line.chars().rev().collect();
        let last_digit = rev_re.find(&rev).unwrap().as_str();
        let last_digit: String = last_digit.chars().rev().collect();
        let last_digit = match last_digit.parse::<i32>() {
            Ok(_) => &last_digit,
            Err(_) => map[&last_digit as &str]
        };
        println!("{} {}", first_digit, last_digit);
        let num_str = first_digit.to_string() + last_digit;
        ans += num_str.parse::<i32>().expect("Unable to parse number");
    }
    println!("ans {}", ans);
}
