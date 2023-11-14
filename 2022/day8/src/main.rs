use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() == 1 { "input.txt" } else { &args[1] };
    let input = fs::read_to_string(file_path).expect("Unable to read file");
    let grid = input.lines()
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            ans = ans.max(scenic_score(&grid, i, j));
        }
    }
    println!("{}", ans);
}

fn scenic_score(grid: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let h = grid[y][x];
    let mut north: usize = 0;
    let mut east: usize = 0;
    let mut south: usize = 0;
    let mut west: usize = 0;
    let mut dist = 0usize;
    for i in (0..y).rev() {
        dist += 1;
        if grid[i][x] >= h {
            break;
        }
    }
    north = dist; dist = 0;
    for i in y+1..grid.len() {
        dist += 1;
        if grid[i][x] >= h {
            break;
        }
    }
    south = dist; dist = 0;
    for i in (0..x).rev() {
        dist += 1;
        if grid[y][i] >= h {
            break;
        }
    }
    east = dist; dist = 0;
    for i in x+1..grid[y].len() {
        dist += 1;
        if grid[y][i] >= h {
            break;
        }
    }
    west = dist;
    north * south * west * east
}
