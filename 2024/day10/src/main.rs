use std::collections::HashSet;
use std::env;
use std::fs;

fn search(y: usize, x: usize, grid: &[Vec<char>], mut path: Vec<(usize, usize)>, paths: &mut HashSet<Vec<(usize, usize)>>) {
    let cur = grid[y][x].to_digit(10).unwrap();
    path.push((y, x));
    if cur == 9 {
        paths.insert(path);
        return;
    }
    let next = char::from_digit(cur + 1, 10).unwrap(); 
    if y > 0 && grid[y - 1][x] == next {
        search(y - 1, x, grid, path.clone(), paths);
    }
    if y < grid.len() - 1 && grid[y + 1][x] == next {
        search(y + 1, x, grid, path.clone(), paths);
    }
    if x > 0 && grid[y][x - 1] == next {
        search(y, x - 1, grid, path.clone(), paths);
    }
    if x < grid.len() - 1 && grid[y][x + 1] == next {
        search(y, x + 1, grid, path.clone(), paths);
    }
}

fn solve(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '0' {
                let mut paths = HashSet::new();
                search(i, j, &grid, Vec::new(), &mut paths);
                ans += paths.len();
            }
        }
    }
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
