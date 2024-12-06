use std::env;
use std::fs;

fn satisfies_rule(rule: &(i32, i32), update: &[i32]) -> bool {
    let mut seen_second = false;
    for page in update {
        if *page == rule.1 {
            seen_second = true;
        } else if *page == rule.0 && seen_second {
            return false;
        }
    }
    true
}

fn satisfies_rules(rules: &[(i32, i32)], update: &[i32]) -> bool {
    for rule in rules {
        if !satisfies_rule(rule, update) {
            return false;
        }
    }
    true
}

// Returns a tuple of indices which indicate the elements that need to be swapped.
// Returns None if no elements need to be swapped.
fn try_sort_update(rules: &[(i32, i32)], update: &[i32]) -> Option<(usize, usize)> {
    let try_sort_helper = |rule: &(i32, i32)| {
        let mut second_ind = None;
        for (i, page) in update.iter().enumerate() {
            if *page == rule.1 {
                second_ind = Some(i);
            } else if *page == rule.0 && second_ind.is_some() {
                return Some((second_ind.unwrap(), i));
            }
        }
        None
    };

    for rule in rules {
        if let Some(val) = try_sort_helper(rule) {
            return Some(val);
        }
    }
    None
}

fn part1(input: &str) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut first_section = true;
    for line in input.lines() {
        if line.is_empty() {
            first_section = false;
            continue;
        }

        if first_section {
            let nums: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
            rules.push((nums[0], nums[1]));
        } else {
            let nums = line.split(',').map(|s| s.parse().unwrap()).collect();
            updates.push(nums);
        }
    }

    let mut ans = 0;
    for update in updates {
        if satisfies_rules(&rules, &update) {
            ans += update[update.len() / 2];
        }
    }
    println!("ans {ans}");
}

fn part2(input: &str) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut first_section = true;
    for line in input.lines() {
        if line.is_empty() {
            first_section = false;
            continue;
        }

        if first_section {
            let nums: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
            rules.push((nums[0], nums[1]));
        } else {
            let nums = line.split(',').map(|s| s.parse().unwrap()).collect();
            updates.push(nums);
        }
    }
    let mut ans = 0;
    for mut update in updates {
        if !satisfies_rules(&rules, &update) {
            // Keep swapping until it is valid
            while let Some((i, j)) = try_sort_update(&rules, &update) {
                update.swap(i, j);
            }
            ans += update[update.len() / 2];
        }
    }
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    part1(&input);
    part2(&input);
}
