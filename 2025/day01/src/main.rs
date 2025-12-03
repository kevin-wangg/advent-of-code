use std::env;
use std::fs;

#[allow(dead_code)]
fn solve1(input: &str) {
    let input: Vec<_> = input.trim().split("\n").collect();
    let mut cur = 50;
    let mut ans = 0;
    for line in input {
        let dir = line.chars().next().unwrap();
        let times = line[1..].parse::<i32>().unwrap() % 100;
        if dir == 'L' {
            cur -= times;
            if cur < 0 {
                cur += 100;
            }
        } else {
            cur += times;
            if cur > 99 {
                cur -= 100;
            }
        }
        if cur == 0 {
            ans += 1;
        }
    }
    println!("ans {ans}");
}

fn solve2(input: &str) {
    let input: Vec<_> = input.trim().split("\n").collect();
    let mut cur = 50;
    let mut ans = 0;
    for line in input {
        let dir = line.chars().next().unwrap();
        let mut times = line[1..].parse::<i32>().unwrap();
        ans += times / 100;
        times = times % 100;
        let mut pass_zero = false;
        let at_zero = cur == 0;
        if dir == 'L' {
            cur -= times;
            if cur < 0 {
                cur += 100;
                pass_zero = true;
            }
        } else {
            cur += times;
            if cur > 99 {
                cur -= 100;
                pass_zero = true;
            }
        }
        if cur == 0 || (!at_zero && pass_zero) {
            ans += 1;
        }
    }
    println!("ans {ans}");
}

fn main() {
    let file = env::args().nth(1).expect("input file should be provided");
    let part = env::args()
        .nth(2)
        .expect("part should be provided (1 or 2)")
        .parse::<i32>()
        .unwrap();
    let input = fs::read_to_string(file).expect("file should exist");
    if part == 1 {
        solve1(&input);
    } else {
        solve2(&input);
    }
}
