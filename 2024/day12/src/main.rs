use std::env;
use std::fs;

fn area(y: i32, x: i32, grid: &[Vec<char>], seen: &mut Vec<Vec<bool>>) -> i32 {
    let diff = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let c = grid[y as usize][x as usize];
    seen[y as usize][x as usize] = true;
    let in_bounds = |y: i32, x: i32| {
        !(y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[0].len())
    };
    let mut ret = 1;
    for d in diff {
        let y = y + d.0;
        let x = x + d.1;
        if in_bounds(y, x) && grid[y as usize][x as usize] == c && !seen[y as usize][x as usize] {
            ret += area(y, x, grid, seen);
        }
    }
    ret
}

fn perimeter(y: i32, x: i32, grid: &[Vec<char>], seen: &mut Vec<Vec<bool>>) -> i32 {
    let diff = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let c = grid[y as usize][x as usize];
    seen[y as usize][x as usize] = true;
    let in_bounds = |y: i32, x: i32| {
        !(y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[0].len())
    };
    let mut ret = 0;
    for d in &diff {
        let y = y + d.0;
        let x = x + d.1;
        if !in_bounds(y, x) || grid[y as usize][x as usize] != c {
            ret += 1
        }
    }
    for d in &diff {
        let y = y + d.0;
        let x = x + d.1;
        if in_bounds(y, x) && grid[y as usize][x as usize] == c && !seen[y as usize][x as usize] {
            ret += perimeter(y, x, grid, seen);
        }
    }
    ret
}

fn corners(y: i32, x: i32, grid: &[Vec<char>], seen: &mut Vec<Vec<bool>>) -> i32 {
    let diff = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let c = grid[y as usize][x as usize];
    seen[y as usize][x as usize] = true;
    let in_bounds = |y: i32, x: i32| {
        !(y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[0].len())
    };
    let filled = |y: i32, x: i32| in_bounds(y, x) && grid[y as usize][x as usize] == c;
    let mut ret = 0;
    let corner_sets = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for c in corner_sets {
        // check out corner
        let new_y = y + c.0;
        let new_x = x + c.1;
        if !filled(new_y, x) && !filled(y, new_x) {
            ret += 1;
        }
        // check in corner
        if !filled(new_y, new_x) && filled(new_y, x) && filled(y, new_x) {
            ret += 1;
        }
    }
    for d in &diff {
        let y = y + d.0;
        let x = x + d.1;
        if in_bounds(y, x) && grid[y as usize][x as usize] == c && !seen[y as usize][x as usize] {
            ret += corners(y, x, grid, seen);
        }
    }
    ret
}

fn solve(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut area_seen: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut perimeter_seen: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if area_seen[i][j] == false {
                let a = area(i as i32, j as i32, &grid, &mut area_seen);
                // let p = perimeter(i as i32, j as i32, &grid, &mut perimeter_seen);
                let c = corners(i as i32, j as i32, &grid, &mut perimeter_seen);
                ans += a * c;
            }
        }
    }
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
