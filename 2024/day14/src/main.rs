use std::env;
use std::fs;

// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7; 
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103; 

const TIME: i32 = 100;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn display_grid(robots: &[Robot]) {
    let mut grid = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '*';
    }
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!();

    }
}

fn solve(input: &str) {
    let mut robots = Vec::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let position: Vec<&str> = line[0].split('=').collect();
        let position: Vec<i32> = position[1].split(',').map(|n| n.parse().unwrap()).collect();
        let speed: Vec<&str> = line[1].split('=').collect();
        let speed: Vec<i32> = speed[1].split(',').map(|n| n.parse().unwrap()).collect();
        robots.push(Robot {
            x: position[0],
            y: position[1],
            vx: speed[0],
            vy: speed[1],
        });
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let midx = WIDTH / 2;
    let midy = HEIGHT / 2;

    for robot in robots {
        let x = (robot.x + (robot.vx * TIME)).rem_euclid(WIDTH);
        let y = (robot.y + (robot.vy * TIME)).rem_euclid(HEIGHT);
        
        if x < midx && y < midy {
            q1 += 1;
        } else if x < midx && y > midy {
            q2 += 1;
        } else if x > midx && y < midy {
            q3 += 1;
        } else if x > midx && y > midy {
            q4 += 1;
        }
    }

    println!("ans {}", q1 * q2 * q3 * q4);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
