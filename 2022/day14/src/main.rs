use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() == 1 {
        "input.txt"
    } else {
        &args[1]
    };
    let input = fs::read_to_string(path).expect("File path should be present");
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    let (max_x, mut max_y) = (1000, 0);
    for line in input.lines() {
        paths.push(
            line.split(" -> ")
                .map(|coord| {
                    let pair = coord.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                    max_y = max_y.max(pair[1]);
                    (pair[0], pair[1])
                })
                .collect::<Vec<(usize, usize)>>()
        );
    }
    paths.push(vec![(0, max_y + 2), (max_x, max_y + 2)]);
    max_y += 2;
    let mut grid: Vec<Vec<char>> = vec![vec!['.';max_x + 1];max_y + 1];
    for p in paths {
        for i in 0..p.len() - 1 {
            let c1 = p[i];
            let c2 = p[i + 1];
            if c1.0 == c2.0 {
                // vertical path
                let start = c1.1.min(c2.1);
                let end = c1.1.max(c2.1);
                for j in start..=end {
                    grid[j][c1.0] = '#';
                }
            } else if c1.1 == c2.1 {
                // horizontal path
                let start = c1.0.min(c2.0);
                let end = c1.0.max(c2.0);
                for j in start..=end {
                    grid[c1.1][j] = '#';
                }
            } else {
                panic!("Diagonal path");
            }
        }
    }
    // for i in 0..grid.len() {
    //     for j in 0..grid[i].len() {
    //         print!("{}", grid[i][j]);
    //     }
    //     println!();
    // }
    // println!();
    let mut ans = 0;
    while spawn_sand(&mut grid) {
        // for i in 0..grid.len() {
        //     for j in 0..grid[i].len() {
        //         print!("{}", grid[i][j]);
        //     }
        //     println!();
        // }
        // println!();
        ans += 1;
    }
    println!("ans {}", ans);
}

fn spawn_sand(grid: &mut Vec<Vec<char>>) -> bool {
    let (mut cur_x, mut cur_y): (usize, usize) = (500, 0);
    if grid[cur_y][cur_x] == 'o' {
        return false;
    }
    grid[cur_y][cur_x] = 'o';
    loop {
        if grid[cur_y + 1][cur_x] == '.' {
            // move down
            grid[cur_y + 1][cur_x] = 'o';
            grid[cur_y][cur_x] = '.';
            cur_y += 1;
        } else if grid[cur_y + 1][cur_x - 1] == '.' {
            // move down left
            grid[cur_y + 1][cur_x - 1] = 'o';
            grid[cur_y][cur_x] = '.';
            cur_y += 1;
            cur_x -= 1;
        } else if grid[cur_y + 1][cur_x + 1] == '.' {
            // move down right
            grid[cur_y + 1][cur_x + 1] = 'o';
            grid[cur_y][cur_x] = '.';
            cur_y += 1;
            cur_x += 1;
        } else {
            break;
        }
    }
    true
}
