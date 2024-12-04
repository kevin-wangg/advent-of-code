use std::collections::HashSet;
use std::env;
use std::fs;

fn is_valid(grid: &[Vec<char>], i: i32, j: i32) -> bool {
    !(i < 0 || j < 0 || i as usize >= grid.len() || j as usize >= grid[i as usize].len()) 
}

fn search(grid: &[Vec<char>], y_delta: i32, x_delta: i32) -> u32 {
    let mut found = 0;
    let word = ['X', 'M', 'A', 'S'];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for (k, c) in word.iter().enumerate() {
                let i = i as i32 + k as i32 * y_delta;
                let j = j as i32 + k as i32 * x_delta;
                if !is_valid(grid, i, j) {
                    break;
                }
                if grid[i as usize][j as usize] != *c {
                    break;
                }
                if k == 3 {
                    found += 1;
                }
            }
        }
    }
    found
}

fn part1(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut ans = 0;
    ans += search(&grid, 0, 1); // right
    ans += search(&grid, 0, -1); // left
    ans += search(&grid, 1, 0); // down
    ans += search(&grid, -1, 0); // up
    ans += search(&grid, 1, 1); // down right
    ans += search(&grid, 1, -1); // down left
    ans += search(&grid, -1, -1); // up left
    ans += search(&grid, -1, 1); // up right
    println!("ans {ans}");
}

fn part2(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'A'
                && is_valid(&grid, i as i32 + 1, j as i32 + 1)
                && is_valid(&grid, i as i32 + 1, j as i32 - 1)
                && is_valid(&grid, i as i32 - 1, j as i32 - 1)
                && is_valid(&grid, i as i32 - 1, j as i32 + 1)
            {
                let expected = HashSet::from(['M', 'S']);
                let mut diag1 = HashSet::new();
                let mut diag2 = HashSet::new();
                diag1.insert(grid[i + 1][j + 1]);
                diag1.insert(grid[i - 1][j - 1]);
                diag2.insert(grid[i + 1][j - 1]);
                diag2.insert(grid[i - 1][j + 1]);
                if diag1 == expected && diag2 == expected {
                    ans += 1;
                }
            }
        }
    }
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    part1(&input);
    part2(&input);
}
