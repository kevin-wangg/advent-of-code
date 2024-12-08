use itertools;
use itertools::Itertools;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn solve(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut coords: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '.' {
                coords
                    .entry(grid[i][j])
                    .and_modify(|v| v.push((i as i32, j as i32)))
                    .or_insert(vec![(i as i32, j as i32)]);
            }
        }
    }
    let valid_position =
        |(y, x): (i32, i32)| y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32;
    let mut positions = HashSet::new();
    for (_k, v) in &coords {
        for pair in v.iter().combinations(2) {
            let y_delta = pair[0]. 0 - pair[1].0;
            let x_delta = pair[0]. 1 - pair[1].1;
            
            let mut p1 = pair[0].clone();
            let mut p2 = pair[1].clone();

            loop {
                if valid_position(p1) {
                    positions.insert(p1);
                    p1.0 -= y_delta;
                    p1.1 -= x_delta;
                } else {
                    break;
                }
            }
            
            loop {
                if valid_position(p2) {
                    positions.insert(p2);
                    p2.0 += y_delta;
                    p2.1 += x_delta;
                } else {
                    break;
                }
            }
        }
    }
    println!("ans {}", positions.len());
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
