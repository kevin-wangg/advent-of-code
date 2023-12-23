use std::{env, fs};
use std::collections::VecDeque;

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let grid: Vec<Vec<usize>> = input.lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
    }).collect();
    let m = grid.len();
    let n = grid[0].len();
    let mut min_loss = vec![vec![vec![vec![vec![vec![1e9 as usize; 4]; 4]; 4]; 4]; n]; m];
    let mut bfs = VecDeque::new();
    bfs.push_back((0, 0, 0, 0, 0, 0));
    min_loss[0][0][0][0][0][0] = 0;
    while let Some((y, x, up_moves, right_moves, down_moves, left_moves)) = bfs.pop_front() {
        let cur_loss = min_loss[y][x][up_moves][right_moves][down_moves][left_moves];
        // println!("cur loss {}", cur_loss);
        if y > 0 {
            if up_moves < 3 && down_moves == 0 {
                if cur_loss + grid[y - 1][x] < min_loss[y - 1][x][up_moves + 1][0][0][0] {
                    min_loss[y - 1][x][up_moves + 1][0][0][0] = cur_loss + grid[y - 1][x];
                    bfs.push_back((y - 1, x, up_moves + 1, 0, 0, 0));
                }
            }
        }
        if y < m - 1 {
            if down_moves < 3 && up_moves == 0 {
                if cur_loss + grid[y + 1][x] < min_loss[y + 1][x][0][0][down_moves + 1][0] {
                    min_loss[y + 1][x][0][0][down_moves + 1][0] = cur_loss + grid[y + 1][x];
                    bfs.push_back((y + 1, x, 0, 0, down_moves + 1, 0));
                }
            }
        }
        if x > 0 {
            if left_moves < 3 && right_moves == 0 {
                if cur_loss + grid[y][x - 1] < min_loss[y][x - 1][0][0][0][left_moves + 1] {
                    min_loss[y][x - 1][0][0][0][left_moves + 1] = cur_loss + grid[y][x - 1];
                    bfs.push_back((y, x - 1, 0, 0, 0, left_moves + 1));
                }
            }
        }
        if x < n - 1 {
            if right_moves < 3 && left_moves == 0 {
                if cur_loss + grid[y][x + 1] < min_loss[y][x + 1][0][right_moves + 1][0][0] {
                    min_loss[y][x + 1][0][right_moves + 1][0][0] = cur_loss + grid[y][x + 1];
                    bfs.push_back((y, x + 1, 0, right_moves + 1, 0, 0));
                }
            }
        }
    }
    let mut ans = 1e9 as usize;
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                for l in 0..4 {
                    println!("{} {} {} {} {}", i, j, k, l, min_loss[m - 1][n - 1][i][j][k][l]);
                    ans = ans.min(min_loss[m - 1][n - 1][i][j][k][l]);
                }
            }
        }
    }
    println!("ans {}", ans);
}
