use std::collections::HashMap;
use std::{env, fs};

fn process(
    x: i32,
    m: i32,
    a: i32,
    s: i32,
    flow: &str,
    workflows: &HashMap<&str, Vec<&str>>,
) -> bool {
    if flow == "A" {
        return true;
    }
    if flow == "R" {
        return false;
    }
    println!("flow {}", flow);
    let wf = &workflows[flow];
    for &f in wf {
        if f.contains(':') {
            let f: Vec<&str> = f.split(':').collect();
            let dest = f[1];
            let c: Vec<char> = f[0].chars().collect();
            let cat = c[0];
            let op = c[1];
            let num: i32 = c
                .iter()
                .filter(|ch| ch.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap();
            let comp = |val: i32| {
                if op == '>' {
                    val > num
                } else if op == '<' {
                    val < num
                } else {
                    panic!("Invalid op")
                }
            };
            match cat {
                'x' => {
                    if comp(x) {
                        return process(x, m, a, s, dest, workflows);
                    }
                }
                'm' => {
                    if comp(m) {
                        return process(x, m, a, s, dest, workflows);
                    }
                }
                'a' => {
                    if comp(a) {
                        return process(x, m, a, s, dest, workflows);
                    }
                }
                's' => {
                    if comp(s) {
                        return process(x, m, a, s, dest, workflows);
                    }
                }
                _ => {
                    panic!("Invalid cat");
                }
            }
        } else {
            if f == "A" {
                return true;
            } else if f == "R" {
                return false;
            } else {
                return process(x, m, a, s, f, workflows);
            }
        }
    }
    false
}

fn get_num(s: &str) -> i32 {
    let s: String = s.chars().filter(|c| c.is_digit(10)).collect();
    s.parse().unwrap()
}
fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let workflows = input[0];
    let workflows: HashMap<&str, Vec<&str>> = workflows
        .lines()
        .map(|line| {
            let line = line.split('{').collect::<Vec<&str>>();
            let name = line[0];
            let rules = line[1][..line[1].len() - 1]
                .split(',')
                .collect::<Vec<&str>>();
            (name, rules)
        })
        .collect();
    println!("{:?}", workflows);
    let parts = input[1];
    let mut ans = 0;
    for p in parts.lines() {
        let p = &p[1..p.len() - 1];
        let p: Vec<&str> = p.split(',').collect();
        let x = get_num(p[0]);
        let m = get_num(p[1]);
        let a = get_num(p[2]);
        let s = get_num(p[3]);
        let accepted = process(x, m, a, s, "in", &workflows);
        if accepted {
            ans += x + m + a + s;
        }
    }
    println!("ans {}", ans);
}
