use std::{env, fs};

fn part1(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let line: Vec<&str> = line.split(':').collect();
        let id = line[0].strip_prefix("Game ").unwrap();
        let sets: Vec<&str> = line[1].split(';').collect();
        let mut valid = true;
        for s in sets {
            let colors: Vec<&str> = s.split(',').collect();
            for c in colors {
                let c: Vec<&str> = c.trim().split_whitespace().collect();
                let num = c[0].parse::<i32>().unwrap();
                let color = c[1];
                if color == "red" {
                    if num > 12 {
                        valid = false;
                    }
                } else if color == "green" {
                    if num > 13 {
                        valid = false;
                    }
                } else if color == "blue" {
                    if num > 14 {
                        valid = false;
                    }
                } else {
                    panic!("Invalid color");
                }
            }
        }
        if valid {
            ans += id.parse::<i32>().unwrap();
        }
    }
    println!("part 1 ans {}", ans);
}

fn part2(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let line: Vec<&str> = line.split(':').collect();
        let sets: Vec<&str> = line[1].split(';').collect();
        let mut minr = 0;
        let mut ming = 0;
        let mut minb = 0;
        for s in sets {
            let colors: Vec<&str> = s.split(',').collect();
            for c in colors {
                let c: Vec<&str> = c.trim().split_whitespace().collect();
                let num = c[0].parse::<i32>().unwrap();
                let color = c[1];
                if color == "red" {
                    minr = minr.max(num);
                } else if color == "green" {
                    ming = ming.max(num);
                } else if color == "blue" {
                    minb = minb.max(num);
                } else {
                    panic!("Invalid color");
                }
            }
        }
        println!("{} {} {}", minr, ming, minb);
        ans += minr * ming * minb;
    }
    println!("part 2 ans {}", ans);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    part1(&input);
    part2(&input);
}
