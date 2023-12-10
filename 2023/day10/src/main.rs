use std::collections::VecDeque;
use std::{env, fs};

enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn valid_move(
    m: Move,
    y: usize,
    x: usize,
    grid: &Vec<Vec<char>>,
    prev_y: usize,
    prev_x: usize,
) -> bool {
    match m {
        Move::Up => {
            let allowed = vec!['|', 'F', '7'];
            if y > 0 {
                let testy = y - 1;
                let testx = x;
                if testy != prev_y || testx != prev_x {
                    for c in allowed {
                        if grid[testy][testx] == c {
                            return true;
                        }
                    }
                }
            }
        }
        Move::Down => {
            let allowed = vec!['|', 'L', 'J'];
            if y < grid.len() {
                let testy = y + 1;
                let testx = x;
                if testy != prev_y || testx != prev_x {
                    for c in allowed {
                        if grid[testy][testx] == c {
                            return true;
                        }
                    }
                }
            }
        }
        Move::Left => {
            let allowed = vec!['-', 'L', 'F'];
            if x > 0 {
                let testy = y;
                let testx = x - 1;
                if testy != prev_y || testx != prev_x {
                    for c in allowed {
                        if grid[testy][testx] == c {
                            return true;
                        }
                    }
                }
            }
        }
        Move::Right => {
            let allowed = vec!['-', 'J', '7'];
            if x < grid[0].len() {
                let testy = y;
                let testx = x + 1;
                if testy != prev_y || testx != prev_x {
                    for c in allowed {
                        if grid[testy][testx] == c {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn mark_loop(start_y: usize, start_x: usize, grid: &Vec<Vec<char>>) -> Option<Vec<Vec<bool>>> {
    let mut y = start_y;
    let mut x = start_x;
    let mut prev_y = y;
    let mut prev_x = x;
    let m = grid.len();
    let n = grid[0].len();
    let mut marked = vec![vec![false; 2 * n + 4]; 2 * m + 4];
    loop {
        let tmpy = y;
        let tmpx = x;
        marked[2 * y + 2][2 * x + 2] = true;
        match grid[y][x] {
            '|' => {
                if valid_move(Move::Up, y, x, grid, prev_y, prev_x) {
                    marked[2 * y - 1 + 2][2 * x + 2] = true;
                    y -= 1;
                } else if valid_move(Move::Down, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 1 + 2][2 * x + 2] = true;
                    y += 1;
                } else {
                    return None;
                }
            }
            '-' => {
                if valid_move(Move::Left, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 2][2 * x - 1 + 2] = true;
                    x -= 1;
                } else if valid_move(Move::Right, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 2][2 * x + 1 + 2] = true;
                    x += 1;
                } else {
                    return None;
                }
            }
            'F' => {
                if valid_move(Move::Down, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 1 + 2][2 * x + 2] = true;
                    y += 1;
                } else if valid_move(Move::Right, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 2][2 * x + 1 + 2] = true;
                    x += 1;
                } else {
                    return None;
                }
            }
            'J' => {
                if valid_move(Move::Up, y, x, grid, prev_y, prev_x) {
                    marked[2 * y - 1 + 2][2 * x + 2] = true;
                    y -= 1;
                } else if valid_move(Move::Left, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 2][2 * x - 1 + 2] = true;
                    x -= 1;
                } else {
                    return None;
                }
            }
            'L' => {
                if valid_move(Move::Up, y, x, grid, prev_y, prev_x) {
                    marked[2 * y - 1 + 2][2 * x + 2] = true;
                    y -= 1;
                } else if valid_move(Move::Right, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 2][2 * x + 1 + 2] = true;
                    x += 1;
                } else {
                    return None;
                }
            }
            '7' => {
                if valid_move(Move::Down, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 1 + 2][2 * x + 2] = true;
                    y += 1;
                } else if valid_move(Move::Left, y, x, grid, prev_y, prev_x) {
                    marked[2 * y + 2][2 * x - 1 + 2] = true;
                    x -= 1;
                } else {
                    return None;
                }
            }
            _ => panic!("Invalid character"),
        }
        if y == start_y && x == start_x {
            break;
        }
        prev_y = tmpy;
        prev_x = tmpx;
    }
    Some(marked)
}

fn count_outside(mut marked: Vec<Vec<bool>>) -> u32 {
    let mut bfs: VecDeque<(usize, usize)> = VecDeque::new();
    // assume 0,0 is outside
    marked[0][0] = true;
    bfs.push_back((0, 0));
    while let Some((y, x)) = bfs.pop_front() {
        if y > 0 {
            if !marked[y - 1][x] {
                marked[y - 1][x] = true;
                bfs.push_back((y - 1, x));
            }
        }
        if y < marked.len() - 1 {
            if !marked[y + 1][x] {
                marked[y + 1][x] = true;
                bfs.push_back((y + 1, x));
            }
        }
        if x > 0 {
            if !marked[y][x - 1] {
                marked[y][x - 1] = true;
                bfs.push_back((y, x - 1));
            }
        }
        if x < marked[0].len() - 1 {
            if !marked[y][x + 1] {
                marked[y][x + 1] = true;
                bfs.push_back((y, x + 1));
            }
        }
    }
    let mut inside = 0;
    for i in 0..marked.len() {
        for j in 0..marked[0].len() {
            if i % 2 == 0 && j % 2 == 0 {
                if !marked[i][j] {
                    inside += 1;
                }
            }
        }
    }
    inside
}

#[allow(dead_code)]
fn print_marked(marked: &Vec<Vec<bool>>) {
    for m in marked {
        for c in m {
            if *c {
                print!("*\t");
            } else {
                print!(".\t");
            }
        }
        println!();
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut y = 0;
    let mut x = 0;
    'outer: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                y = i;
                x = j;
                break 'outer;
            }
        }
    }
    let possible = vec!['|', '-', 'L', 'J', '7', 'F'];
    for p in possible {
        grid[y][x] = p;
        match mark_loop(y, x, &grid) {
            Some(marked) => {
                println!("ans {}", count_outside(marked));
                break;
            }
            None => {}
        }
    }
}
