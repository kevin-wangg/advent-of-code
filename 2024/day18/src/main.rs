use std::collections::VecDeque;
use std::env;
use std::fs;

// const LENGTH: usize = 7;
const LENGTH: usize = 71;

// const BYTES: usize = 12;
const BYTES: usize = 1024;

fn solve(input: &str) {
    let mut grid = vec![vec!['.'; LENGTH]; LENGTH];

    for line in input.lines().take(BYTES) {
        let coord: Vec<usize> = line.split(',').map(|n| n.parse().unwrap()).collect();
        grid[coord[1]][coord[0]] = '#';
    }

    let mut queue = VecDeque::new();
    let mut distance = vec![vec![-1; LENGTH]; LENGTH];
    queue.push_back((0, 0));
    distance[0][0] = 0;

    let valid_pos =
        |x: i32, y: i32| x >= 0 && y >= 0 && (x as usize) < LENGTH && (y as usize) < LENGTH;

    let diff = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

    while let Some((x, y)) = queue.pop_front() {
        for d in &diff {
            let newx = x as i32 + d.0;
            let newy = y as i32 + d.1;
            if valid_pos(newx, newy)
                && grid[newx as usize][newy as usize] != '#'
                && distance[newx as usize][newy as usize] == -1
            {
                distance[newx as usize][newy as usize] = distance[x][y] + 1;
                queue.push_back((newx as usize, newy as usize));
            }
        }
    }

    let ans = distance[LENGTH - 1][LENGTH - 1];
    println!("ans {}", ans);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
