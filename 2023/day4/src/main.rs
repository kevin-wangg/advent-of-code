use std::{env, fs};
use std::collections::HashSet;

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let lines: Vec<&str> = input.lines().collect();
    let mut mult = vec![1; lines.len()];
    for (i, &line) in lines.iter().enumerate() {
        let line: Vec<&str> = line.split(':').collect();
        let nums = line[1].trim();
        // println!("{}", nums);
        let nums: Vec<&str> = nums.split('|').collect();
        // println!("{:?}", nums);
        let winning: HashSet<i32> = nums[0].trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        let mine: HashSet<i32> = nums[1].trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        println!("{:?} {:?}", winning, mine);
        let matching = winning.intersection(&mine).count() as usize;
        for j in i + 1..i + matching + 1 {
            mult[j] += mult[i];
        }
    }
    let ans: i32 = mult.iter().sum();
    println!("ans {}", ans);
}
