use std::env;
use std::fs;

fn solve(input: &str) {
    let inputs: Vec<&str> = input.split("\n\n").collect();
    let mut grid: Vec<Vec<char>> = inputs[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let moves = inputs[1];
    let mut start = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                start = (i, j);
                break;
            }
        }
    }
    for c in moves.chars() {
        if !c.is_whitespace() {
            start = do_move(start.0, start.1, &mut grid, c);
        }
    }

    print(&grid);
    println!("ans {}", score(&grid));
}

fn check_move(y: usize, x: usize, grid: &mut Vec<Vec<char>>, c: char) -> bool {
    if grid[y][x] == '.' {
        true
    } else if grid[y][x] == '#' {
        false
    } else {
        let next = match c {
            '>' => (y, x + 1),
            '<' => (y, x - 1),
            '^' => (y - 1, x),
            'v' => (y + 1, x),
            _ => panic!("invalid move {}", c),
        };
        if check_move(next.0, next.1, grid, c) {
            grid[next.0][next.1] = grid[y][x];
            true
        } else {
            false
        }
    }
}

fn do_move(y: usize, x: usize, grid: &mut Vec<Vec<char>>, c: char) -> (usize, usize) {
    if check_move(y, x, grid, c) {
        grid[y][x] = '.';
        match c {
            '>' => (y, x + 1),
            '<' => (y, x - 1),
            '^' => (y - 1, x),
            'v' => (y + 1, x),
            _ => panic!("invalid move {}", c),
        }
    } else {
        (y, x)
    }
}

fn score(grid: &[Vec<char>]) -> i32 {
    let mut ret = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                ret += 100 * i as i32 + j as i32;
            }
        }
    }
    ret
}

fn print(grid: &[Vec<char>]) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
