use std::{env, fs};
use std::collections::HashMap;
use regex::Regex;
use num::Integer;

struct Steps {
    count: usize,
    s: Vec<char>,
}

impl Steps {
    fn new(s: &str) -> Self {
        Steps {
            count: 0,
            s: s.chars().collect()
        }
    }

    fn next(&mut self) -> char {
        let ret = self.s[self.count % self.s.len()];
        self.count += 1;
        ret
    }

    fn reset(&mut self) {
        self.count = 0;
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let input: Vec<&str> = input.lines().filter(|&l| !l.is_empty()).collect();
    let mut steps = Steps::new(input[0]);
    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    let re = Regex::new(r"[a-zA-Z1-9]+").unwrap();
    let mut ans: i64 = 1;
    let mut ghosts: Vec<&str> = vec![];
    for i in 1..input.len() {
        let nodes: Vec<&str> = re.find_iter(input[i]).map(|m| m.as_str()).collect();
        network.insert(nodes[0], (nodes[1], nodes[2]));
        if nodes[0].chars().last().unwrap() == 'A' {
            ghosts.push(nodes[0]);
        }
    }

    for g in ghosts {
        let mut cur = g;
        while cur.chars().last().unwrap() != 'Z' {
            let next = steps.next();
            if next == 'L' {
                cur = network[cur].0;
            } else {
                cur = network[cur].1;
            }
        }
        ans = ans.lcm(&(steps.count as i64));
        steps.reset();
    }
    println!("ans {}", ans);
}
