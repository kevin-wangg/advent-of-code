use std::env;
use std::fs;

fn solve(input: &str) {
    let inputs: Vec<&str> = input.split("\n\n").collect();
    let grid: Vec<Vec<char>> = inputs[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut grid = expand_grid(grid);

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
            start = process_move(start.0, start.1, &mut grid, c);
        }
    }

    println!("ans {}", score(&grid));
}

fn expand_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ret = vec![Vec::new(); grid.len()];
    for i in 0..grid.len() {
        for c in &grid[i] {
            match c {
                '#' => {
                    ret[i].push('#');
                    ret[i].push('#');
                }
                'O' => {
                    ret[i].push('[');
                    ret[i].push(']');
                }
                '.' => {
                    ret[i].push('.');
                    ret[i].push('.');
                }
                '@' => {
                    ret[i].push('@');
                    ret[i].push('.');
                }
                _ => panic!("invalid character {}", c),
            }
        }
    }
    ret
}

fn check_move(y: usize, x: usize, grid: &mut Vec<Vec<char>>, c: char, check_beside: bool) -> bool {
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
        let mut ok = check_move(next.0, next.1, grid, c, true);
        if (c == '^' || c == 'v') && grid[y][x] != '@' && check_beside {
            if grid[y][x] == '[' {
                ok &= check_move(y, x + 1, grid, c, false);
            } else if grid[y][x] == ']' {
                ok &= check_move(y, x - 1, grid, c, false);
            } else {
                panic!("invalid box character {}", grid[y][x]);
            }
        }
        ok
    }
}

fn do_move(y: usize, x: usize, grid: &mut Vec<Vec<char>>, c: char, check_beside: bool) {
    if grid[y][x] == '.' {
        return;
    } else if grid[y][x] == '#' {
        return;
    } else {
        let next = match c {
            '>' => (y, x + 1),
            '<' => (y, x - 1),
            '^' => (y - 1, x),
            'v' => (y + 1, x),
            _ => panic!("invalid move {}", c),
        };
        do_move(next.0, next.1, grid, c, true);
        if (c == '^' || c == 'v') && grid[y][x] != '@' && check_beside {
            if grid[y][x] == '[' {
                do_move(y, x + 1, grid, c, false);
                grid[y][x + 1] = '.';
            } else if grid[y][x] == ']' {
                do_move(y, x - 1, grid, c, false);
                grid[y][x - 1] = '.';
            } else {
                panic!("invalid box character {}", grid[y][x]);
            }
        }
        grid[next.0][next.1] = grid[y][x];
    }
}

fn process_move(y: usize, x: usize, grid: &mut Vec<Vec<char>>, c: char) -> (usize, usize) {
    if check_move(y, x, grid, c, true) {
        do_move(y, x, grid, c, true);
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
            if grid[i][j] == '[' {
                ret += 100 * i as i32 + j as i32;
            }
        }
    }
    ret
}

#[allow(dead_code)]
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
