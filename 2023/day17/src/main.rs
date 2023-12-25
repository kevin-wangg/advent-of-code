use std::collections::VecDeque;
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let m = grid.len();
    let n = grid[0].len();
    let mut min_loss = vec![vec![vec![vec![vec![vec![1e9 as usize; 11]; 11]; 11]; 11]; n]; m];
    let mut bfs = VecDeque::new();
    bfs.push_back((0, 0, 0, 0, 0, 0));
    min_loss[0][0][0][0][0][0] = 0;
    while let Some((y, x, up_moves, right_moves, down_moves, left_moves)) = bfs.pop_front() {
        let cur_loss = min_loss[y][x][up_moves][right_moves][down_moves][left_moves];
        if up_moves < 10 && down_moves == 0 {
            if y > 0 && up_moves > 0 {
                min_loss[y - 1][x][up_moves + 1][0][0][0] = cur_loss + grid[y - 1][x];
                bfs.push_back((y - 1, x, up_moves + 1, 0, 0, 0));
            } else if y > 3
                && cur_loss + grid[y - 4][x] + grid[y - 3][x] + grid[y - 2][x] + grid[y - 1][x]
                    < min_loss[y - 4][x][up_moves + 4][0][0][0]
            {
                min_loss[y - 4][x][up_moves + 4][0][0][0] =
                    cur_loss + grid[y - 4][x] + grid[y - 3][x] + grid[y - 2][x] + grid[y - 1][x];
                bfs.push_back((y - 4, x, up_moves + 4, 0, 0, 0));
            }
        }
        if down_moves < 10 && up_moves == 0 {
            if y < m - 1 && down_moves > 0 {
                min_loss[y + 1][x][0][0][down_moves + 1][0] = cur_loss + grid[y + 1][x];
                bfs.push_back((y + 1, x, 0, 0, down_moves + 1, 0));
            } else if y < m - 4
                && cur_loss + grid[y + 4][x] + grid[y + 3][x] + grid[y + 2][x] + grid[y + 1][x]
                    < min_loss[y + 4][x][0][0][down_moves + 4][0]
            {
                min_loss[y + 4][x][0][0][down_moves + 4][0] =
                    cur_loss + grid[y + 4][x] + grid[y + 3][x] + grid[y + 2][x] + grid[y + 1][x];
                bfs.push_back((y + 4, x, 0, 0, down_moves + 4, 0));
            }
        }
        if left_moves < 10 && right_moves == 0 {
            if x > 0 && left_moves > 0 {
                min_loss[y][x - 1][0][0][0][left_moves + 1] = cur_loss + grid[y][x - 1];
                bfs.push_back((y, x - 1, 0, 0, 0, left_moves + 1));
            } else if x > 3
                && cur_loss + grid[y][x - 4] + grid[y][x - 3] + grid[y][x - 2] + grid[y][x - 1]
                    < min_loss[y][x - 4][0][0][0][left_moves + 4]
            {
                min_loss[y][x - 4][0][0][0][left_moves + 4] =
                    cur_loss + grid[y][x - 4] + grid[y][x - 3] + grid[y][x - 2] + grid[y][x - 1];
                bfs.push_back((y, x - 4, 0, 0, 0, left_moves + 4));
            }
        }
        if right_moves < 10 && left_moves == 0 {
            if x < n - 1 && right_moves > 0 {
                min_loss[y][x + 1][0][right_moves + 1][0][0] = cur_loss + grid[y][x + 1];
                bfs.push_back((y, x + 1, 0, right_moves + 1, 0, 0));
            } else if x < n - 4
                && cur_loss + grid[y][x + 4] + grid[y][x + 3] + grid[y][x + 2] + grid[y][x + 1]
                    < min_loss[y][x + 4][0][right_moves + 4][0][0]
            {
                min_loss[y][x + 4][0][right_moves + 4][0][0] =
                    cur_loss + grid[y][x + 4] + grid[y][x + 3] + grid[y][x + 2] + grid[y][x + 1];
                bfs.push_back((y, x + 4, 0, right_moves + 4, 0, 0));
            }
        }
    }
    let mut ans = 1e9 as usize;
    for i in 0..11 {
        for j in 0..11 {
            for k in 0..11 {
                for l in 0..11 {
                    ans = ans.min(min_loss[m - 1][n - 1][i][j][k][l]);
                }
            }
        }
    }
    println!("ans {}", ans);
}
