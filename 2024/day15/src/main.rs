use std::env;

use std::fs;

fn do_move(y: usize, x: usize, grid: &mut Vec<Vec<char>>, c: char) -> (usize, usize) {
    match c {
        '>' => {
            let mut cnt = 1;
            loop {
                if grid[y][x + cnt] == '#' {
                    return (y, x);
                }
                if grid[y][x + cnt] == '.' {
                    break;
                }
                cnt += 1;
            }
            let end = x + cnt;
            for i in (x..end).rev() {
                grid[y][i + 1] = grid[y][i];
            }
            grid[y][x] = '.';
            (y, x + 1)
        }
        '<' => {}
        '^' => {}
        'v' => {}
        _ => {}
    }
}

fn solve(input: &str) {
    let inputs: Vec<&str> = input.split("\n\n").collect();
    let grid: Vec<Vec<char>> = inputs[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Remember to ignore newlines
    let moves = inputs[1];
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
