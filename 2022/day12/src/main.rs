use std::{fs, env, collections::VecDeque};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() == 1 {
        "input.txt"
    } else {
        &args[1]
    };
    let input = fs::read_to_string(path).expect("Unable to read file");
    let mut grid: Vec<Vec<char>> = input.lines()
        .map(|line| {
            line.chars().collect::<Vec<_>>()
        })
        .collect();
    let mut desty: usize = 0;
    let mut destx: usize = 0;
    let mut bfs: VecDeque<(i32, i32)> = VecDeque::new();
    let mut dist: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' || grid[i][j] == 'a' {
                bfs.push_front((i as i32, j as i32));
                dist[i][j] = 0;
                grid[i][j] = 'a';
            } else if grid[i][j] == 'E' {
                desty = i;
                destx = j;   
                grid[i][j] = 'z';
            }
        }
    }
    let delta: Vec<(i32, i32)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0)
    ];
    while !bfs.is_empty() {
        let top = bfs.pop_back().expect("Queue is empty");
        for d in &delta {
            let newy = top.0 + d.0;
            let newx = top.1 + d.1;
            if newy >= 0 && (newy as usize) < grid.len() && newx >= 0 && (newx as usize) < grid[0].len() {
                if dist[newy as usize][newx as usize] == -1 {
                    let newh = grid[newy as usize][newx as usize];
                    let curh = grid[top.0 as usize][top.1 as usize];
                    let diff = newh as i32 - curh as i32;
                    if diff <= 1 {
                        dist[newy as usize][newx as usize] = dist[top.0 as usize][top.1 as usize] + 1;
                        bfs.push_front((newy, newx));
                    }
                }
            }
        }
    }
    println!("{}", dist[desty][destx]);
}
