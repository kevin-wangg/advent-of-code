use std::env;
use std::fs;

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 1 {
        true
    } else {
        let asc = report[1] > report[0];
        let (low_bound, high_bound) = if asc { (1, 3) } else { (-3, -1) };
        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            if diff < low_bound || diff > high_bound {
                return false;
            }
        }
        true
    }
}

fn part1(input: &str) {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(levels);
    }
    let mut safe_count = 0;
    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        }
    }
    println!("safe count {safe_count}");
}

fn part2(input: &str) {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(levels);
    }
    let mut safe_count = 0;
    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if is_safe(&new_report) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    println!("safe count {safe_count}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exists");
    part1(&input);
    part2(&input);
}
