use std::env;
use std::fs;

use regex::Regex;

fn solve_case(case: &[&str]) -> Option<i64> {
    let re = Regex::new(r"\d+").unwrap();
    let mut iter = re.find_iter(case[0]);
    let ax = iter.next().unwrap().as_str().parse::<f64>().unwrap();
    let ay = iter.next().unwrap().as_str().parse::<f64>().unwrap();
    let mut iter = re.find_iter(case[1]);
    let bx = iter.next().unwrap().as_str().parse::<f64>().unwrap();
    let by = iter.next().unwrap().as_str().parse::<f64>().unwrap();
    let mut iter = re.find_iter(case[2]);
    let mut px = iter.next().unwrap().as_str().parse::<f64>().unwrap();
    let mut py = iter.next().unwrap().as_str().parse::<f64>().unwrap();

    px += 10000000000000.0;
    py += 10000000000000.0;

    const A_COST: i64 = 3;
    const B_COST: i64 = 1;

    let b = (py - (ay / ax * px)) / (by - (ay / ax * bx));
    let a = (px / ax) - (bx / ax * b);
    let a = a.round() as i64;
    let b = b.round() as i64;
    if a >= 0
        && b >= 0
        && (ax as i64 * a + bx as i64 * b) == px as i64
        && (ay as i64 * a + by as i64 * b) == py as i64
    {
        Some(a * A_COST + b * B_COST)
    } else {
        None
    }
}

fn solve(input: &str) {
    let mut case = Vec::new();
    let mut ans = 0;
    for line in input.lines() {
        if line.len() == 0 {
            solve_case(&case).inspect(|tokens| {
                ans += tokens;
            });
            case.clear();
        } else {
            case.push(line);
        }
    }
    solve_case(&case).inspect(|tokens| {
        ans += tokens;
    });
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
