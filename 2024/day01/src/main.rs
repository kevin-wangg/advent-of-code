use std::collections::HashMap;
use std::env;
use std::fs;

fn part1(input: &str) {
    let mut distance = 0;
    let mut list1 = Vec::new(); 
    let mut list2 = Vec::new(); 
    for line in input.lines() {
        let chars: Vec<&str> = line.split_whitespace().collect();
        let n1 = chars[0].parse::<i32>().unwrap();
        let n2 = chars[1].parse::<i32>().unwrap();
        list1.push(n1);
        list2.push(n2);
    }
    list1.sort();
    list2.sort();
    for i in 0..list1.len() {
        distance += (list1[i] - list2[i]).abs();
    }
    println!("distance {distance}");
}

fn part2(input: &str) {
    let mut score = 0;
    let mut list = Vec::new();
    let mut occurrences = HashMap::new();
    for line in input.lines() {
        let chars: Vec<&str> = line.split_whitespace().collect();
        let n1 = chars[0].parse::<i32>().unwrap();
        let n2 = chars[1].parse::<i32>().unwrap();
        list.push(n1);
        occurrences.entry(n2).and_modify(|v| *v += 1).or_insert(1);
    }
    for n in list {
        score += n * occurrences.get(&n).unwrap_or(&0);
    }
    println!("score {score}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exists");
    part1(&input);
    part2(&input);
}
