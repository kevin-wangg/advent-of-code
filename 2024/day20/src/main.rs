use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn bfs(grid: &[Vec<char>]) -> Vec<Vec<i32>> {
    let mut start = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start = (i as i32, j as i32);
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

    distance
}

const LIMIT: i32 = 20;

fn shortcut(grid: &[Vec<char>], distance: &[Vec<i32>], start: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut visited = vec![vec![-1; grid[0].len()]; grid.len()];
    visited[start.0][start.1] = 0;

    let diff: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut end_pos = HashSet::new();
    while let Some((y, x)) = queue.pop_front() {
        if visited[y][x] > LIMIT {
            continue;
        }

        if distance[y][x] - distance[start.0][start.1] - visited[y][x] >= 100 {
            end_pos.insert((y, x));
        }

        for d in &diff {
            let next = (y as i32 + d.0, x as i32 + d.1);
            let next = (next.0 as usize, next.1 as usize);
            if next.0 > 0 && next.1 > 0 && next.0 < grid.len() - 1 && next.1 < grid[0].len() - 1 {
                if visited[next.0][next.1] == -1 {
                    visited[next.0][next.1] = visited[y][x] + 1;
                    queue.push_back(next);
                }
            }
        }
    }
    end_pos.len()
}

fn solve(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let distance = bfs(&grid);
    let mut ans = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if distance[i][j] != -1 {
                ans += shortcut(&grid, &distance, (i, j));
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
