use std::collections::HashSet;
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let plan: Vec<(&str, i32, &str)> = input
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split_whitespace().collect();
            (l[0], l[1].parse().unwrap(), l[2])
        })
        .collect();
    let (mut y, mut x) = (0, 0);
    let mut miny = 0;
    let mut minx = 0;
    let mut maxy = 0;
    let mut maxx = 0;
    for &(dir, steps, _) in &plan {
        match dir {
            "U" => {
                y -= steps;
            }
            "R" => {
                x += steps;
            }
            "L" => {
                x -= steps;
            }
            "D" => {
                y += steps;
            }
            _ => {
                panic!("Invalid direction");
            }
        }
        miny = miny.min(y);
        maxy = maxy.max(y);
        minx = minx.min(x);
        maxx = maxx.max(x);
    }
    y = 0;
    x = 0;
    let mut points = HashSet::new();
    points.insert((y + i32::abs(miny), x + i32::abs(minx)));
    for &(dir, steps, _) in &plan {
        match dir {
            "U" => {
                for _ in 0..steps {
                    y -= 1;
                    points.insert((y + i32::abs(miny), x + i32::abs(minx)));
                }
            }
            "R" => {
                for _ in 0..steps {
                    x += 1;
                    points.insert((y + i32::abs(miny), x + i32::abs(minx)));
                }
            }
            "L" => {
                for _ in 0..steps {
                    x -= 1;
                    points.insert((y + i32::abs(miny), x + i32::abs(minx)));
                }
            }
            "D" => {
                for _ in 0..steps {
                    y += 1;
                    points.insert((y + i32::abs(miny), x + i32::abs(minx)));
                }
            }
            _ => {
                panic!("Invalid direction");
            }
        }
    }
    let mut ans = 0;
    for i in 0..=i32::abs(miny) + maxy {
        let mut include = false;
        let mut start = 0;
        let mut end = 0;
        for j in 0..=i32::abs(minx) + maxx {
            // println!("{} {} {}", i, j, include);
            if include || points.contains(&(i, j)) {
                ans += 1;
            }

            if !points.contains(&(i, j - 1)) && points.contains(&(i, j)) {
                if points.contains(&(i + 1, j)) && points.contains(&(i - 1, j)) {
                    include = !include;
                } else if points.contains(&(i + 1, j)) {
                    start = 1;
                } else if points.contains(&(i - 1, j)) {
                    start = -1;
                }
            } else if points.contains(&(i, j)) && !points.contains(&(i, j + 1)) {
                if points.contains(&(i + 1, j)) {
                    end = 1;
                } else if points.contains(&(i - 1, j)) {
                    end = -1;
                }

                if start != end {
                    include = !include;
                }
            }
        }
    }
    println!("ans {}", ans);
}
