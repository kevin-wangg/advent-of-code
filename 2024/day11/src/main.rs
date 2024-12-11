use std::collections::HashMap;
use std::env;
use std::fs;

const LIMIT: i64 = 75;

fn blink(stone: i64, times: i64, dp: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(val) = dp.get(&(stone, times)) {
        return *val;
    }
    if times >= LIMIT {
        1
    } else if stone == 0 {
        let ret = blink(1, times + 1, dp);
        dp.insert((stone, times), ret);
        ret

    } else if stone.to_string().len() % 2 == 0 {
        let s = stone.to_string();
        let stone1: i64 = s[0..s.len() / 2].parse().unwrap();
        let stone2: i64 = s[s.len() / 2..s.len()].parse().unwrap();
        let ret = blink(stone1, times + 1, dp) + blink(stone2, times + 1, dp);
        dp.insert((stone, times), ret);
        ret
    } else {
        let ret = blink(stone * 2024, times + 1, dp);
        dp.insert((stone, times), ret);
        ret
    }
}

fn solve(input: &str) {
    let stones: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut ans = 0;
    let mut dp = HashMap::new();
    for stone in stones {
        ans += blink(stone, 0, &mut dp);
    }
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
