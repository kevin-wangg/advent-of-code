use std::{env, fs};

fn valid_spot(y: usize, x: usize, grid: &Vec<Vec<char>>, vis: &Vec<Vec<bool>>, allowed: Vec<char>) -> bool {
    if vis[y][x] {
        return false;
    }
    for a in allowed {
        if a == grid[y][x] {
            return true;
        }
    }
    false
}

fn loop_length(start_y: usize, start_x: usize, grid: &Vec<Vec<char>>, m: usize, n: usize) -> Option<usize> {
    let mut vis = vec![vec![false; n]; m];
    let mut cur_y = start_y;
    let mut cur_x = start_x;
    let mut first = true;
    let mut length = 0;
    while first || cur_y != start_y || cur_x != start_x {
        println!("cur y {} cur x {}", cur_y, cur_x);
        first = false;
        vis[cur_y][cur_x] = true;
        if length == 2 {
            vis[start_y][start_x] = false;
        }
        match grid[cur_y][cur_x] {
            '|' => {
                if cur_y > 0 && valid_spot(cur_y - 1, cur_x, grid, &vis, vec!['F', '7', '|']) {
                    cur_y -= 1;
                } else if cur_y < m - 1 && valid_spot(cur_y + 1, cur_x, grid, &vis, vec!['|', 'L', 'J']) {
                    cur_y += 1;
                } else {
                    println!("RETURN NONE");
                    return None;
                }
            },
            '-' => {
                if cur_x > 0 && valid_spot(cur_y, cur_x - 1, grid, &vis, vec!['-', 'F', 'L']) {
                    cur_x -= 1;
                } else if cur_x < n - 1 && valid_spot(cur_y, cur_x + 1, grid, &vis, vec!['-', 'J', '7']) {
                    cur_x += 1;
                } else {
                    println!("RETURN NONE");
                    return None;
                }
            },
            'L' => {
                if cur_y > 0 && valid_spot(cur_y - 1, cur_x, grid, &vis, vec!['F', '7', '|']) {
                    cur_y -= 1;
                } else if cur_x < n - 1 && valid_spot(cur_y, cur_x + 1, grid, &vis, vec!['-', 'J', '7']) {
                    cur_x += 1;
                } else {
                    println!("RETURN NONE");
                    return None;
                }
            },
            'J' => {
                if cur_y > 0 && valid_spot(cur_y - 1, cur_x, grid, &vis, vec!['F', '7', '|']) {
                    cur_y -= 1;
                } else if cur_x > 0 && valid_spot(cur_y, cur_x - 1, grid, &vis, vec!['-', 'F', 'L']) {
                    cur_x -= 1;
                } else {
                    println!("RETURN NONE");
                    return None;
                }
            },
            '7' => {
                if cur_y < m - 1 && valid_spot(cur_y + 1, cur_x, grid, &vis, vec!['|', 'L', 'J']) {
                    cur_y += 1;
                } else if cur_x > 0 && valid_spot(cur_y, cur_x - 1, grid, &vis, vec!['-', 'F', 'L']) {
                    cur_x -= 1;
                } else {
                    println!("RETURN NONE");
                    return None;
                }
            },
            'F' => {
                if cur_y < m - 1 && valid_spot(cur_y + 1, cur_x, grid, &vis, vec!['|', 'L', 'J']) {
                    cur_y += 1;
                } else if cur_x < n - 1 && valid_spot(cur_y, cur_x + 1, grid, &vis, vec!['-', 'J', '7']) {
                    cur_x += 1;
                } else {
                    println!("RETURN NONE");
                    return None;
                }
            },
            _ => panic!("Invalid character found")
        }
        length += 1;
    }
    Some(length)
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| {
        l.chars().collect()
    }).collect();
    let m = grid.len();
    let n = grid[0].len();
    let mut start_y = 0;
    let mut start_x = 0;
    'outer: for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'S' {
                start_y = i;
                start_x = j;
                break 'outer;
            }
        }
    }
    let possible = vec!['|', '-', 'L', 'J', '7', 'F'];
    // S is '-'
    for p in possible {
        grid[start_y][start_x] = p;
        println!("setting to p {}", p);
        match loop_length(start_y, start_x, &grid, m, n) {
            Some(length) => {
                println!("ans {}", length / 2);
                println!("{}", p);
                break;
            },
            None => {}
        }
    }
}
