use std::env;
use std::fs;
use std::io;

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
    let mut grid = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] += 1;
    }
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                print!(".");
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
}

fn contains_straight_line(robots: &[Robot]) -> bool {
    let mut grid = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] += 1;
    }
    let length = 5;
    for i in 0..grid.len() - length {
        for j in 0..grid[i].len() {
            let mut ok = true;
            for k in 0..length {
                if grid[i + k][j] == 0 {
                    ok = false;
                    break;
                }
            }
            if ok {
                return true;
            }
        }
    }
    false
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

    let mut time = 0;

    let midx = WIDTH / 2;
    let midy = HEIGHT / 2;

    loop {
        println!("time {time}");

        let mut q1: i32 = 0;
        let mut q2: i32 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        for robot in &mut robots {
            robot.x = (robot.x + robot.vx).rem_euclid(WIDTH);
            robot.y = (robot.y + robot.vy).rem_euclid(HEIGHT);

            if robot.x < midx && robot.y < midy {
                q1 += 1;
            } else if robot.x < midx && robot.y > midy {
                q2 += 1;
            } else if robot.x > midx && robot.y < midy {
                q3 += 1;
            } else if robot.x > midx && robot.y > midy {
                q4 += 1;
            }
        }

        if contains_straight_line(&robots) {
            display_grid(&robots);
            let mut buf = String::new();
            let _ = io::stdin().read_line(&mut buf);
        }

        time += 1;
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
