use std::{env, fs};
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    Up, Left, Down, Right
}

fn calc_energized(grid: &Vec<Vec<char>>, y: i32, x: i32, d: Dir) -> usize {
    let mut dfs: VecDeque<(i32, i32, Dir)> = VecDeque::new();
    dfs.push_front((y, x, d));
    let mut seen = HashSet::new();
    let mut energized = HashSet::new();
    while let Some((y, x, d)) = dfs.pop_front() {
        if seen.contains(&(y, x, d)) {
            continue;
        }
        seen.insert((y, x, d));
        energized.insert((y, x));
        match d {
            Dir::Up => {
                if y > 0 {
                    let next = grid[(y - 1) as usize][x as usize];
                    if next == '|' || next == '.' {
                        dfs.push_front((y - 1, x, Dir::Up));
                    } else if next == '\\' {
                        dfs.push_front((y - 1, x, Dir::Left));
                    } else if next == '/' {
                        dfs.push_front((y - 1, x, Dir::Right));
                    } else if next == '-' {
                        dfs.push_front((y - 1, x, Dir::Left));
                        dfs.push_front((y - 1, x, Dir::Right));
                    } else {
                        panic!("Invalid character");
                    }
                }
            },
            Dir::Left => {
                if x > 0 {
                    let next = grid[y as usize][(x - 1) as usize];
                    if next == '|' {
                        dfs.push_front((y, x - 1, Dir::Up));
                        dfs.push_front((y, x - 1, Dir::Down));
                    } else if next == '\\' {
                        dfs.push_front((y, x - 1, Dir::Up));
                    } else if next == '/' {
                        dfs.push_front((y, x - 1, Dir::Down));
                    } else if next == '.' || next == '-' {
                        dfs.push_front((y, x - 1, Dir::Left));
                    } else {
                        panic!("Invalid character");
                    }
                }
            },
            Dir::Down => {
                if y < (grid.len() - 1) as i32 {
                    let next = grid[(y + 1) as usize][x as usize];
                    if next == '|' || next == '.' {
                        dfs.push_front((y + 1, x, Dir::Down));
                    } else if next == '\\' {
                        dfs.push_front((y + 1, x, Dir::Right));
                    } else if next == '/' {
                        dfs.push_front((y + 1, x, Dir::Left));
                    } else if next == '-' {
                        dfs.push_front((y + 1, x, Dir::Left));
                        dfs.push_front((y + 1, x, Dir::Right));
                    } else {
                        panic!("Invalid character");
                    }
                }
            },
            Dir::Right => {
                if x < (grid[0].len() - 1) as i32 {
                    let next = grid[y as usize][(x + 1) as usize];
                    if next == '|' {
                        dfs.push_front((y, x + 1, Dir::Up));
                        dfs.push_front((y, x + 1, Dir::Down));
                    } else if next == '\\' {
                        dfs.push_front((y, x + 1, Dir::Down));
                    } else if next == '/' {
                        dfs.push_front((y, x + 1, Dir::Up));
                    } else if next == '.' || next == '-' {
                        dfs.push_front((y, x + 1, Dir::Right));
                    } else {
                        panic!("Invalid character");
                    }
                }
            }
        }
    }
    energized.len() - 1
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let grid: Vec<Vec<char>> = input.lines().map(|l| {
        l.chars().collect()
    }).collect();
    let mut ans = 0;
    for i in 0..grid.len() {
        ans = ans.max(calc_energized(&grid, i as i32, -1, Dir::Right));
        ans = ans.max(calc_energized(&grid, i as i32, grid[0].len() as i32, Dir::Left));
    }
    for i in 0..grid[0].len() {
        ans = ans.max(calc_energized(&grid, -1, i as i32, Dir::Down));
        ans = ans.max(calc_energized(&grid, grid.len() as i32, i as i32, Dir::Up));
    }
    println!("ans {ans}");
}
