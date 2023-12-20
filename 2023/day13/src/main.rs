use std::{env, fs};

const HORIZONTAL_MUL: usize = 100;

fn rows_equal(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    grid[i] == grid[j]
}

fn cols_equal(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    for k in 0..grid.len() {
        if grid[k][i] != grid[k][j] {
            return false;
        }
    }
    true
}

fn horizontal(grid: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..grid.len() - 1 {
        if rows_equal(grid, i, i + 1) {
            let mut l = i;
            let mut r = i + 1;
            loop {
                if !rows_equal(grid, l, r) {
                    break;
                }
                if l == 0 || r == grid.len() - 1 {
                    return Some(i);
                }
                l -= 1;
                r += 1;
            }

        }
    }
    None
}

fn vertical(grid: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..grid[0].len() - 1 {
        if cols_equal(grid, i, i + 1) {
            let mut l = i;
            let mut r = i + 1;
            loop {
                if !cols_equal(grid, l, r) {
                    break;
                }
                if l == 0 || r == grid[0].len() - 1 {
                    return Some(i);
                }
                l -= 1;
                r += 1;
            }

        }
    }
    None
}

fn reflection(grid: &Vec<Vec<char>>) -> usize {
    if let Some(line) = horizontal(grid) {
        return HORIZONTAL_MUL * (line + 1);
    }

    if let Some(line) = vertical(grid) {
        return line + 1;
    }
    panic!("Grid has no reflections");

}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut ans = 0;
    for grid in input.split("\n\n") {
        let grid: Vec<Vec<char>> = grid.lines().map(|l| {
            l.chars().collect()
        }).collect();
        ans += reflection(&grid);
    }
    println!("ans {}", ans);
}
