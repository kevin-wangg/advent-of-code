use std::{env, fs};

fn shoelace(v: &[(i64, i64)]) -> i64 {
    let n = v.len();
    let mut sum = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        let add = (v[i].0 * v[j].1) - (v[i].1 * v[j].0);
        sum += add;
    }
    sum.abs() / 2
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let plan: Vec<(char, i64)> = input
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split_whitespace().collect();
            // (l[0].chars().nth(0).unwrap(), l[1].parse().unwrap())
            let color = l[2];
            let color = &color[2..color.len() - 1];
            let dir = match color.chars().last().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("Invalid character!"),
            };
            let steps = i64::from_str_radix(&color[0..color.len() - 1], 16).unwrap();
            (dir, steps)
        })
        .collect();
    let mut p = (0, 0);
    let mut v = vec![];
    let mut ans = 0;
    v.push(p);
    for (dir, steps) in plan {
        match dir {
            'U' => {
                p.1 -= steps;
                ans += steps;
            }
            'R' => {
                p.0 += steps;
            }
            'D' => {
                p.1 += steps;
            }
            'L' => {
                p.0 -= steps;
                ans += steps;
            }
            _ => {
                panic!("Invalid direction!");
            }
        }
        v.push(p);
    }
    ans += shoelace(&v);
    println!("ans {}", ans + 1);
}
