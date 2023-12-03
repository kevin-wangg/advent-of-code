use std::{env, fs};
use std::collections::VecDeque;

fn is_engine(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn check_for_number(y: usize, x: usize, vis: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>) -> i32 {
    if !grid[y][x].is_numeric() || vis[y][x] {
        return 0;
    }
    let mut num = VecDeque::new();
    for i in x..grid[y].len() {
        if !grid[y][i].is_numeric() || vis[y][i] {
            break;
        }
        num.push_back(grid[y][i]);
        vis[y][i] = true;
    }
    for i in (0..x).rev() {
        if !grid[y][i].is_numeric() || vis[y][i] {
            break;
        }
        num.push_front(grid[y][i]);
        vis[y][i] = true;
    }
    let num_str: String = num.iter().collect();
    num_str.parse().unwrap()
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut vis: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_engine(grid[i][j]) {
                ans += check_for_number(i + 1, j, &mut vis, &grid);
                ans += check_for_number(i - 1, j, &mut vis, &grid);
                ans += check_for_number(i, j + 1, &mut vis, &grid);
                ans += check_for_number(i, j - 1, &mut vis, &grid);
                ans += check_for_number(i + 1, j + 1, &mut vis, &grid);
                ans += check_for_number(i + 1, j - 1, &mut vis, &grid);
                ans += check_for_number(i - 1, j + 1, &mut vis, &grid);
                ans += check_for_number(i - 1, j - 1, &mut vis, &grid);
            }
        }
    }
    println!("ans {}", ans);
}
