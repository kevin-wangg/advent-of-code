use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn mix(secret_number: i64, val: i64) -> i64 {
    secret_number ^ val
}

fn prune(secret_number: i64) -> i64 {
    secret_number % 16777216
}

fn next(secret_number: i64) -> i64 {
    let secret_number = prune(mix(secret_number, secret_number * 64));
    let secret_number = prune(mix(secret_number, secret_number / 32));
    let secret_number = prune(mix(secret_number, secret_number * 2048));
    secret_number
}

fn last_digit(n: i64) -> i64 {
    n % 10
}

fn solve(input: &str) {
    let mut all: HashMap<VecDeque<i64>, i64> = HashMap::new();
    for line in input.lines() {
        let mut sequence: HashMap<VecDeque<i64>, i64> = HashMap::new();
        let mut n = line.parse::<i64>().unwrap();
        let mut cur_seq = VecDeque::new();
        let mut prev = last_digit(n);
        for _ in 0..4 {
            n = next(n);
            cur_seq.push_back(last_digit(n) - prev);
            prev = last_digit(n);
        }
        sequence.insert(cur_seq.clone(), last_digit(n));
        for _ in 0..1996 {
            n = next(n);
            cur_seq.push_back(last_digit(n) - prev);
            cur_seq.pop_front();
            prev = last_digit(n);
            sequence.entry(cur_seq.clone()).or_insert(last_digit(n));
        }

        for (k, v) in sequence {
            *all.entry(k).or_insert(0) += v;
        }
    }
    println!("ans {}", all.values().max().unwrap());
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
