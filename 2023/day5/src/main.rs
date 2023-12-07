use std::{env, fs};

fn get_initial(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .filter_map(|w| w.parse::<i64>().ok())
        .collect()
}

fn get_location(mut s: i64, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    // println!("initial s {}", s);
    for m in maps {
        for r in m {
            if s >= r[1] && s <= r[1] + r[2] {
                // Within start range
                let diff = s - r[1];
                // Set s to new value
                s = r[0] + diff;
                // println!("changed s {}", s);
                break;
            }
        }
    }
    s
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let input: Vec<&str> = input.lines().collect();
    let initial = get_initial(input[0]);
    let mut maps: Vec<_> = input.split(|&l| {
        l.is_empty()
    }).collect();
    maps.remove(0);
    let maps: Vec<Vec<Vec<i64>>> = maps.iter().map(|&m| {
        let m = &m[1..m.len()];
        m.iter().map(|&l| {
            l.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect()
        }).collect()
    }).collect();
    let mut ans = 1e9 as i64;
    for s in initial {
        let l = get_location(s, &maps);
        // println!("final loc: {}", l);
        ans = ans.min(l);
    }
    println!("{}", ans);
}
