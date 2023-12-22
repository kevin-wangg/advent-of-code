use std::{env, fs};

fn hash(s: &str) -> usize {
    let mut cur = 0;
    for c in s.chars() {
        if c == '\n' {
            continue;
        }
        cur += c as usize;
        cur *= 17;
        cur %= 256;
    }
    cur
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exists");
    let input: String = input.chars().filter(|&c| c != '\n').collect();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for s in input.split(',') {
        if s.contains('=') {
            let s: Vec<&str> = s.split('=').collect();
            let label = s[0];
            let len = s[1].parse::<usize>().unwrap();
            let b = hash(label);
            if let Some(i) = boxes[b].iter().position(|&lens| lens.0 == label) {
                boxes[b][i].1 = len;
            } else {
                boxes[b].push((label, len));
            }
        } else if s.contains('-') {
            let label = &s[0..s.len() - 1];
            let b = hash(label);
            boxes[b].retain(|&lens| lens.0 != label);
        } else {
            panic!("Invalid instruction");
        }
    }
    let mut ans = 0;
    for b in 0..256 {
        for (i, &lens) in boxes[b].iter().enumerate() {
            ans += (b + 1) * (i + 1) * lens.1;
        }
    }
    println!("ans {ans}");
}
