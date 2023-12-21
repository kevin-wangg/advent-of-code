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

fn horizontal(grid: &Vec<Vec<char>>, exclude: &Vec<usize>) -> Option<usize> {
    for i in 0..grid.len() - 1 {
        if rows_equal(grid, i, i + 1) && !exclude.contains(&i) {
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

fn vertical(grid: &Vec<Vec<char>>, exclude: &Vec<usize>) -> Option<usize> {
    for i in 0..grid[0].len() - 1 {
        if cols_equal(grid, i, i + 1) && !exclude.contains(&i) {
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

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut ans = 0;
    'outer: for grid in input.split("\n\n") {
        let mut grid: Vec<Vec<char>> = grid.lines().map(|l| {
            l.chars().collect()
        }).collect();
        let mut exclude_vert = vec![];
        let mut exclude_horz = vec![];
        if let Some(line) = horizontal(&grid, &vec![]) {
            exclude_horz.push(line);
        } else if let Some(line) = vertical(&grid, &vec![]) {
            exclude_vert.push(line);
        } else {
            panic!("No reflection found");
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let tmp = grid[i][j];
                if grid[i][j] == '.' {
                    grid[i][j] = '#';
                } else {
                    grid[i][j] = '.';
                }
                if let Some(line) = horizontal(&grid, &exclude_horz) {
                    ans += HORIZONTAL_MUL * (line + 1);
                    continue 'outer;
                } else if let Some(line) = vertical(&grid, &exclude_vert) {
                    ans += line + 1;
                    continue 'outer;
                }
                grid[i][j] = tmp;
            }
        }
        panic!("No second reflection found");
    }
    println!("ans {}", ans);
}
