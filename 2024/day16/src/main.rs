mod default_map;

use std::env;
use std::fs;
use std::i32;

use default_map::DefaultMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn turn_left(&self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}
use Dir::*;

fn dfs(
    grid: &[Vec<char>],
    scores: &mut DefaultMap<(usize, usize, Dir), i32>,
    y: usize,
    x: usize,
    dir: Dir,
) {
    let forward = match dir {
        North => (y - 1, x),
        East => (y, x + 1),
        West => (y, x - 1),
        South => (y + 1, x),
    };

    if grid[forward.0][forward.1] != '#' {
        let cur_score = scores[(y, x, dir)];
        if cur_score + 1 < scores.get_or_default((forward.0, forward.1, dir), i32::MAX) {
            scores[(forward.0, forward.1, dir)] = cur_score + 1;
            dfs(grid, scores, forward.0, forward.1, dir);
        }
    }

    let left = match dir {
        North => (y, x - 1),
        East => (y - 1, x),
        West => (y + 1, x),
        South => (y, x + 1),
    };

    let right = match dir {
        North => (y, x + 1),
        East => (y + 1, x),
        West => (y - 1, x),
        South => (y, x - 1),
    };

    for next in vec![left, right] {
        let new_dir = if next == left {
            dir.turn_left()
        } else {
            dir.turn_right()
        };

        if grid[next.0][next.1] != '#' {
            let cur_score = scores[(y, x, dir)];
            if cur_score + 1001 < scores.get_or_default((next.0, next.1, new_dir), i32::MAX) {
                scores[(next.0, next.1, new_dir)] = cur_score + 1001;
                dfs(grid, scores, next.0, next.1, new_dir);
            }
        }
    }
}

fn solve(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut min_score: DefaultMap<(usize, usize, Dir), i32> = DefaultMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start = (i, j);
            }
            if grid[i][j] == 'E' {
                end = (i, j);
            }
        }
    }

    min_score[(start.0, start.1, Dir::East)] = 0;
    dfs(&grid, &mut min_score, start.0, start.1, Dir::East);

    let ans = i32::min(
        i32::min(
            min_score.get_or_default((end.0, end.1, North), i32::MAX),
            min_score.get_or_default((end.0, end.1, East), i32::MAX),
        ),
        i32::min(
            min_score.get_or_default((end.0, end.1, South), i32::MAX),
            min_score.get_or_default((end.0, end.1, West), i32::MAX),
        ),
    );
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
