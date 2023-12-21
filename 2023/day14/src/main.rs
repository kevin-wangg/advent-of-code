use std::{env, fs};
use std::collections::HashMap;

const CYCLES: usize = 1_000_000_000;

enum Dir {
    North, West, South, East
}

fn tilt(grid: &mut Vec<Vec<char>>, dir: Dir) {
    match dir {
        Dir::North => {
            for i in 0..grid[0].len() {
                let mut pos = 0;
                for j in 0..grid.len() {
                    if grid[j][i] == 'O' {
                        grid[j][i] = '.';
                        grid[pos][i] = 'O';
                        pos += 1;
                    } else if grid[j][i] == '#' {
                        pos = j + 1;
                    }
                }
            }
        },
        Dir::West => {
            for i in 0..grid.len() {
                let mut pos = 0;
                for j in 0..grid[0].len() {
                    if grid[i][j] == 'O' {
                        grid[i][j] = '.';
                        grid[i][pos] = 'O';
                        pos += 1;
                    } else if grid[i][j] == '#' {
                        pos = j + 1;
                    }
                }
            }
        },
        Dir::South => {
            for i in 0..grid[0].len() {
                let mut pos: i32 = grid.len() as i32 - 1;
                for j in (0..grid.len()).rev() {
                    if grid[j][i] == 'O' {
                        grid[j][i] = '.';
                        grid[pos as usize][i] = 'O';
                        pos -= 1;
                    } else if grid[j][i] == '#' {
                        pos = j as i32 - 1;
                    }
                }
            }
        },
        Dir::East => {
            for i in 0..grid.len() {
                let mut pos: i32 = grid[0].len() as i32 - 1;
                for j in (0..grid[0].len()).rev() {
                    if grid[i][j] == 'O' {
                        grid[i][j] = '.';
                        grid[i][pos as usize] = 'O';
                        pos -= 1;
                    } else if grid[i][j] == '#' {
                        pos = j as i32 - 1;
                    }
                }
            }
        }
    }
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    tilt(grid, Dir::North);
    tilt(grid, Dir::West);
    tilt(grid, Dir::South);
    tilt(grid, Dir::East);
}

#[allow(unused)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for r in grid {
        for c in r {
            print!("{c}");
        }
        println!();
    }
    println!();
}

fn calc_north_load(grid: &Vec<Vec<char>>) -> usize {
    let mut ret = 0;
    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            if grid[j][i] == 'O' {
                ret += grid.len() - j;
            }
        }
    }
    ret
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("Path should exist");
    let mut grid: Vec<Vec<char>>= input.lines().map(|l| {
        l.chars().collect()
    }).collect();
    let mut seen = HashMap::new();
    let mut cycle_len;
    let mut cnt = 0;
    loop {
        if cnt >= CYCLES { break; }
        cycle(&mut grid);
        if seen.contains_key(&grid) {
            cycle_len = cnt - seen[&grid];
            let times = (CYCLES - cnt) / cycle_len;
            cnt += times * cycle_len;
        }
        seen.insert(grid.clone(), cnt);
        cnt += 1;
    }
    let ans = calc_north_load(&grid);
    println!("ans {}", ans);
}
