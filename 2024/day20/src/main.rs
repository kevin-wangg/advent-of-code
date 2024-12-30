use std::collections::VecDeque;
use std::env;
use std::fs;

fn bfs(grid: &[Vec<char>]) -> u32 {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start = (i as i32, j as i32);
            }
            if grid[i][j] == 'E' {
                end = (i as i32, j as i32);
            }
        }
    }
    let mut queue = VecDeque::new();
    let mut distance = vec![vec![-1; grid[0].len()]; grid.len()];
    queue.push_back(start);
    distance[start.0 as usize][start.1 as usize] = 0;
    let diff = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((y, x)) = queue.pop_front() {
        for d in &diff {
            let next = (y + d.0, x + d.1);
            if grid[next.0 as usize][next.1 as usize] != '#'
                && distance[next.0 as usize][next.1 as usize] == -1
            {
                queue.push_back(next);
                distance[next.0 as usize][next.1 as usize] = distance[y as usize][x as usize] + 1;
            }
        }
    }

    distance[end.0 as usize][end.1 as usize] as u32
}

fn solve(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let no_save = bfs(&grid);
    let mut ans = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] == '#'
                && ((grid[i - 1][j] == '.' && grid[i + 1][j] == '.')
                    || (grid[i][j - 1] == '.' && grid[i][j + 1] == '.'))
            {
                let mut grid = grid.clone();
                grid[i][j] = '.';
                let new_time = bfs(&grid);
                if no_save - new_time >= 100 {
                    ans += 1;
                }
            }
        }
    }
    println!("ans {}", ans);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
