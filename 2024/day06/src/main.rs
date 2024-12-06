use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn next(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}

fn part1(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut y = 0;
    let mut x = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                y = i as i32;
                x = j as i32;
                break;
            }
        }
    }
    let mut visited = HashSet::new();
    visited.insert((y, x));

    let mut dir = Dir::North;
    let valid = |y: i32, x: i32| {
        !(y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[0].len())
    };
    loop {
        visited.insert((y, x));
        match dir {
            Dir::North => {
                if valid(y - 1, x) {
                    if grid[y as usize - 1][x as usize] != '#' {
                        y -= 1;
                    } else {
                        dir = dir.next();
                    }
                } else {
                    break;
                }
            }
            Dir::East => {
                if valid(y, x + 1) {
                    if grid[y as usize][x as usize + 1] != '#' {
                        x += 1;
                    } else {
                        dir = dir.next();
                    }
                } else {
                    break;
                }
            }
            Dir::South => {
                if valid(y + 1, x) {
                    if grid[y as usize + 1][x as usize] != '#' {
                        y += 1;
                    } else {
                        dir = dir.next();
                    }
                } else {
                    break;
                }
            }
            Dir::West => {
                if valid(y, x - 1) {
                    if grid[y as usize][x as usize - 1] != '#' {
                        x -= 1;
                    } else {
                        dir = dir.next();
                    }
                } else {
                    break;
                }
            }
        }
    }
    println!("distinct positions: {}", visited.len());
}

fn check_for_loop(grid: &[Vec<char>], mut y: i32, mut x: i32) -> bool {
    let mut visited = HashSet::new();
    let mut dir = Dir::North; 
    let valid = |y: i32, x: i32| {
        !(y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[0].len())
    };
    loop {
        if visited.contains(&(y, x, dir)) {
            return true;
        }
        visited.insert((y, x, dir));
        match dir {
            Dir::North => {
                if valid(y - 1, x) {
                    if grid[y as usize - 1][x as usize] != '#' {
                        y -= 1;
                    } else {
                        dir = dir.next();
                    }
                } else {
                    break;
                }
            }
            Dir::East => {
                if valid(y, x + 1) {
                    if grid[y as usize][x as usize + 1] != '#' {
                        x += 1;
                    } else {
                        dir = dir.next();
                    }
                } else {
                    break;
                }
            }
            Dir::South => {
                if valid(y + 1, x) {
                    if grid[y as usize + 1][x as usize] != '#' {
                        y += 1;
                    } else {
                        dir = dir.next();
                    }
                } else {
                    break;
                }
            }
            Dir::West => {
                if valid(y, x - 1) {
                    if grid[y as usize][x as usize - 1] != '#' {
                        x -= 1;
                    } else {
                        dir = dir.next();
                    }
                } else {
                    break;
                }
            }
        }
    }
    false
}

fn part2(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut y = 0;
    let mut x = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                y = i as i32;
                x = j as i32;
                break;
            }
        }
    }
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                let mut grid = grid.clone();
                grid[i][j] = '#';
                if check_for_loop(&grid, y, x) {
                    ans += 1;
                }
            }
        }
    }
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File path should exist");
    part1(&input);
    part2(&input);
}
