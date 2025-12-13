use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file = &args[1];
    let part: i32 = args[2].parse().unwrap();
    let input = fs::read_to_string(file).expect("file should exist");
    if part == 1 {
        solve1(&input);
    } else {
        solve2(&input);
    }
}

fn solve1(input: &str) {
    let mut ans = 0;
    for range in input.trim().split(",") {
        let range: Vec<i64> = range.split("-").map(|n| n.parse().unwrap()).collect();
        let start = range[0];
        let end = range[1];
        for i in start..=end {
            let s = i.to_string();
            if s[0..s.len() / 2] == s[s.len() / 2..s.len()] {
                ans += i;
            }
        }
    }
    println!("ans {ans}");
}

fn solve2(input: &str) {
    let mut ans = 0;
    for range in input.trim().split(",") {
        let range: Vec<i64> = range.split("-").map(|n| n.parse().unwrap()).collect();
        let start = range[0];
        let end = range[1];
        for i in start..=end {
            let s = i.to_string();
            'outer: for length in 1..=s.len() / 2 {
                if s.len() % length != 0 {
                    continue;
                }
                let pat = &s[0..length];
                let mut j = length;
                while j < s.len() {
                    if &s[j..j + length] != pat {
                        continue 'outer;
                    }
                    j += length;
                }
                ans += i;
                break;
            }
        }
    }
    println!("ans {ans}");
}
