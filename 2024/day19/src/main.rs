use std::collections::HashMap;
use std::env;
use std::fs;

fn ways(design: &str, towels: &[&str], seen: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        1
    } else if seen.contains_key(design) {
        seen[design]
    } 
    else {
        let mut ret = 0;
        for t in towels {
            if design.starts_with(t) {
                let remaining = &design[t.len()..];
                ret += ways(remaining, towels, seen);
            }
        }
        seen.insert(design.to_string(), ret);
        ret
    }
}

fn solve(input: &str) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let towels: Vec<&str> = input[0].split(", ").collect();
    let mut ans = 0;
    let mut seen = HashMap::new();
    for design in input[1].lines() {
        ans += ways(design, &towels, &mut seen);
    }

    println!("ans {}", ans);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
