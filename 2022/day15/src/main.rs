use std::{env, fs};
use regex::Regex;

struct Sensor {
    p: (i64, i64),
    dist: i64
}

impl Sensor {
    fn new(x: i64, y: i64, dist: i64) -> Self {
        Sensor {
            p: (x, y), dist
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (target_row, path) = if args.len() == 1 {
        (2000000, "input.txt")
    } else {
        (10, "sample.txt")
    };
    let coord_max = target_row * 2;
    let re = Regex::new(r"-?\d+").unwrap();
    let input = fs::read_to_string(path).expect("File path should exist");
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in input.lines() {
        let c: Vec<i64> = re.find_iter(line).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
        let dist = manhattan_dist(&(c[0], c[1]), &(c[2], c[3]));
        sensors.push(
            Sensor::new(c[0], c[1], dist)
        )
    }
    let mut beacon: (i64, i64) = (0, 0);
    for s in &sensors {
        for i in 0..=s.dist + 1 {
            let j = s.dist + 1 - i;
            if valid_point(&(s.p.0 + i, s.p.1 + j), &sensors) {
                if in_range(coord_max, &(s.p.0 + i, s.p.1 + j)) {
                    beacon = (s.p.0 + i, s.p.1 + j);
                }
            }
            if valid_point(&(s.p.0 - i, s.p.1 + j), &sensors) {
                if in_range(coord_max, &(s.p.0 - i, s.p.1 + j)) {
                    beacon = (s.p.0 - i, s.p.1 + j);
                }
            }
            if valid_point(&(s.p.0 + i, s.p.1 - j), &sensors) {
                if in_range(coord_max, &(s.p.0 + i, s.p.1 - j)) {
                    beacon = (s.p.0 + i, s.p.1 - j);
                }
            }
            if valid_point(&(s.p.0 - i, s.p.1 - j), &sensors) {
                if in_range(coord_max, &(s.p.0 - i, s.p.1 - j)) {
                    beacon = (s.p.0 - i, s.p.1 - j);
                }
            }
        }
    }
    println!("{:?}", beacon);
    let ans = beacon.0 * 4_000_000 + beacon.1;
    println!("ans {}", ans);
}

fn in_range(coord_max: i64, p: &(i64, i64)) -> bool {
    p.0 >= 0 && p.0 <= coord_max && p.1 >= 0 && p.1 <= coord_max
}

fn manhattan_dist(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    i64::abs(p1.0 - p2.0) + i64::abs(p1.1 - p2.1)
}

fn valid_point(p: &(i64, i64), sensors: &[Sensor]) -> bool {
    for s in sensors {
        if manhattan_dist(p, &s.p) <= s.dist {
            return false;
        }
    }
    true
}
