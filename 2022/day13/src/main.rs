use std::{env, fs, collections::VecDeque};
use std::cmp::Ordering;

enum Decision {
    Known(bool),
    Unknown
}

use crate::Decision::{Known, Unknown};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() == 1 {
        "input.txt"
    } else {
        &args[1]
    };
    let input = fs::read_to_string(path).expect("File should exist");
    let mut packets: Vec<&str> = input
        .lines()
        .filter(|&l| {
            !l.is_empty()
        })
        .collect();
    let divider1 = "[[2]]";
    let divider2 = "[[6]]";
    packets.push(divider1);
    packets.push(divider2);
    packets.sort_by(|&a, &b| {
        let o = compare_packets(a, b);
        match o {
            Known(b) => {
                if b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            Unknown => {
                Ordering::Equal
            }
        }
    });
    let i1 = packets.iter().position(|&p| p == divider1).unwrap() + 1;
    let i2 = packets.iter().position(|&p| p == divider2).unwrap() + 1;
    let ans = i1 * i2;
    println!("ans {}", ans);

}

fn parse_list(s: &str) -> VecDeque<String> {
    let mut ret = VecDeque::new();
    // Remove leading and trailing square brackets
    let s = &s[1..s.len() - 1];
    if s.len() == 0 {
        return ret;
    }
    let mut cur = String::from("");
    let mut brackets = 0;
    for c in s.chars() {
        if c == ',' {
            if brackets == 0 {
                ret.push_back(cur.clone());
                cur.clear();
                continue;
            } 
        } else if c == '[' {
            brackets += 1;
        } else if c == ']' {
            brackets -= 1;
        }
        cur.push(c);
    }
    ret.push_back(cur);
    ret
}

fn compare_packets(p1: &str, p2: &str) -> Decision {
    let r1 = p1.parse::<i32>();
    let r2 = p2.parse::<i32>();
    match r1 {
        Ok(n1) => {
            match r2 {
                Ok(n2) => {
                    if n1 < n2 {
                        Known(true)
                    } else if n1 > n2 {
                        Known(false)
                    } else {
                        Unknown 
                    }
                },
                Err(_) => {
                    let l = format!("[{}]", n1);
                    compare_packets(&l, p2)
                }
            }
        },
        Err(_) => {
            match r2 {
                Ok(n2) => {
                    let l = format!("[{}]", n2);
                    compare_packets(p1, &l)
                },
                Err(_) => {
                    let mut v1 = parse_list(p1);
                    let mut v2 = parse_list(p2);
                    while !v1.is_empty() && !v2.is_empty() {
                        let t1 = v1.pop_front().unwrap();
                        let t2 = v2.pop_front().unwrap();
                        match compare_packets(&t1, &t2) {
                            Known(b) => {
                                return Known(b);
                            },
                            Unknown => {}
                        }
                    }

                    if v1.is_empty() && !v2.is_empty() {
                        Known(true)
                    } else if !v1.is_empty() && v2.is_empty() {
                        Known(false)
                    } else {
                        Unknown
                    }
                }
            }
        }
    }
}