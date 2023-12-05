use std::{env, fs};

fn get_initial(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|w| w.parse::<i32>().ok())
        .collect()
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
    let maps: Vec<Vec<Vec<i32>>> = maps.iter().map(|&m| {
        let m = &m[1..m.len()];
        m.iter().map(|&l| {
            l.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect()
        }).collect()
    }).collect();
    println!("{:?}", initial);
    for m in &maps {
        println!("{:?}", m);
    }
}
