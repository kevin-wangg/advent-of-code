use std::{env, fs};
use std::collections::VecDeque;

const EXPAND: i64 = 1_000_000;

fn valid(grid: &Vec<Vec<char>>, dists: &Vec<Vec<i64>>, y: i64, x: i64) -> bool {
    if y >= 0 && (y as usize) < grid.len() && x >= 0 && (x as usize) < grid[0].len() && dists[y as usize][x as usize] == -1 {
        true
    } else {
        false
    }
}

fn shortest_path_sums(grid: &Vec<Vec<char>>, expand_rows: &Vec<bool>, expand_cols: &Vec<bool>, y: usize, x: usize) -> i64 {
    let mut dists = vec![vec![-1; grid[0].len()]; grid.len()];
    dists[y][x] = 0;
    let mut bfs = VecDeque::new();
    let delta = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0)
    ];
    let mut ret = 0;
    bfs.push_back((y as i64, x as i64));
    while let Some((y, x)) = bfs.pop_front() {
        for d in &delta {
            let newy = y + d.0;
            let newx = x + d.1;
            if valid(grid, &dists, newy, newx) {
                dists[newy as usize][newx as usize] = dists[y as usize][x as usize] + 1;
                if expand_rows[newy as usize] {
                    dists[newy as usize][newx as usize] += EXPAND - 1;
                }
                if expand_cols[newx as usize] {
                    dists[newy as usize][newx as usize] += EXPAND - 1;
                }
                if grid[newy as usize][newx as usize] == '#' {
                    ret += dists[newy as usize][newx as usize];
                }
                bfs.push_back((newy, newx));
            }
        }
    }
    ret
}

fn get_expand_rows(grid: &Vec<Vec<char>>) -> Vec<bool> {
    let mut ret = Vec::new();
    for r in grid {
        if r.contains(&'#') {
            ret.push(false);
        } else {
            ret.push(true);
        }
    }
    ret
}

fn get_expand_cols(grid: &Vec<Vec<char>>) -> Vec<bool> {
    let mut ret = Vec::new();
    'outer: for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            if grid[j][i] == '#' {
                ret.push(false);
                continue 'outer;
            }
        }
        ret.push(true);
    }
    ret
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let expand_rows = get_expand_rows(&grid);
    let expand_cols = get_expand_cols(&grid);
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                ans += shortest_path_sums(&grid, &expand_rows, &expand_cols, i, j);
                grid[i][j] = '.';
            }
        }
    }
    println!("ans {}", ans);
}
