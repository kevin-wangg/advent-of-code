use std::env;
use std::fs;

fn generate_sequences(limit: usize, current: Vec<char>, accum: &mut Vec<Vec<char>>) {
    if current.len() == limit {
        accum.push(current);
    } else {
        let mut new_vec = current.clone();
        new_vec.push('+');
        generate_sequences(limit, new_vec, accum);
        let mut new_vec = current.clone();
        new_vec.push('*');
        generate_sequences(limit, new_vec, accum);
        let mut new_vec = current.clone();
        new_vec.push('|');
        generate_sequences(limit, new_vec, accum);
    }
}

fn get_value(nums: &[i64], ops: &[char]) -> i64 {
    let mut cur = nums[0];
    for i in 1..nums.len() {
        if ops[i - 1] == '+' {
            cur += nums[i];
        } else if ops[i - 1] == '*' {
            cur *= nums[i];
        } else if ops[i - 1] == '|' { // For part 2. Remove this else if for part 1
            cur = format!("{}{}", cur, nums[i]).parse().unwrap(); 
        }
    }
    cur
}

fn solve(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let line: Vec<&str> = line.split(':').collect();
        let target = line[0].parse::<i64>().unwrap();
        let nums: Vec<i64> = line[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if nums.len() == 1 {
            if nums[0] == target {
                ans += target;
            }
        } else {
            let mut sequences = Vec::new();
            generate_sequences(nums.len() - 1, Vec::new(), &mut sequences);
            for seq in &sequences {
                if get_value(&nums, seq) == target {
                    ans += target;
                    break;
                }
            }
        }
    }
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
