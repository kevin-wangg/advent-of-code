use std::{fs, env};
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    op: Vec<String>,
    test: i64,
    b1: usize,
    b2: usize,
}


const MOD: i64 = 9699690;

fn main() {
    const ROUNDS: usize = 10000;
    let args: Vec<String> = env::args().collect();
    let path = if args.len() == 1 {
        "input.txt"
    } else {
        &args[1]
    };
    let input: String = fs::read_to_string(path)
        .expect("Unable to read file");
    let mut monkeys: Vec<Monkey> = Vec::new();
    for m in input.split("\n\n") {
        monkeys.push(make_monkey(m))
    }
    let mut inspections: Vec<i64> = vec![0; monkeys.len()];

    for r in 0..ROUNDS {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop_front().expect("Popped item from empty list");
                // println!("monkey {} inspect item with worry {}", i, item);
                item = process_op(&monkeys[i].op, item);
                // println!("monkey gets bored, worry is now {}", item);
                item %= MOD;
                let b1 = monkeys[i].b1;
                let b2 = monkeys[i].b2;
                if item % monkeys[i].test == 0 {
                    // println!("thrown to monkey {}", b1);
                    monkeys[b1].items.push_back(item);
                } else {
                    // println!("thrown to monkey {}", b2);
                    monkeys[b2].items.push_back(item);
                }
                inspections[i] += 1;
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    println!("{:?}", inspections[0] * inspections[1]);
}

fn process_op(op: &Vec<String>, old: i64) -> i64 {
    let mut operation = "+";
    op.iter()
        .map(|s| {
            if s == "old" {
                old.to_string()
            } else {
                s.to_string()
            }
        })
        .fold(0, |acc, s| {
            if s == "+" {
                operation = "+";
                acc
            } else if s == "*" {
                operation = "*";
                acc
            } else {
                if operation == "+" {
                    acc + s.parse::<i64>().unwrap()
                } else {
                    acc * s.parse::<i64>().unwrap()
                }
            }
        })
}

fn make_starting_items(line: &str) -> VecDeque<i64> {
    line.strip_prefix("  Starting items: ")
        .expect("Unable to parse starting items")
        .split(", ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn make_op(line: &str) -> Vec<String> {
    line.strip_prefix("  Operation: new = ")
        .expect("Unable to parse operation")
        .split(' ')
        .map(|s| s.into())
        .collect()
}

fn get_last_number(line: &str) -> i64 {
    line.split_whitespace()
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .parse()
        .expect("Unable to parse last number")
}


fn make_monkey(lines: &str) -> Monkey {
    let lines: Vec<&str> = lines.lines().collect();
    Monkey {
        items: make_starting_items(lines[1]),
        op: make_op(lines[2]),
        test: get_last_number(lines[3]),
        b1: get_last_number(lines[4]) as usize,
        b2: get_last_number(lines[5]) as usize
    }
}