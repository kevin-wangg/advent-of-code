use std::{env, fs};
use std::collections::VecDeque;

fn all_zeros(a: &[i32]) -> bool {
    for n in a {
        if *n != 0 {
            return false;
        }
    }
    true
}

fn transform(a: &[i32]) -> Vec<i32> {
    let mut ret = vec![];
    for i in 1..a.len() {
        ret.push(a[i] - a[i - 1]);
    }
    ret
}

fn extrapolate(mut a: Vec<i32>) -> i32 {
    let mut stack: VecDeque<i32> = VecDeque::new();
    while !all_zeros(&a) {
        stack.push_front(*a.first().unwrap()); 
        a = transform(&a);
    }
    let mut add = 0;
    while let Some(top) = stack.pop_front() {
        add = top - add;
    }
    add
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut ans = 0;
    for line in input.lines() {
        let nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        let ex = extrapolate(nums);
        ans += ex;
    }
    println!("ans {}", ans);
}
