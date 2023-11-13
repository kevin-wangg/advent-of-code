use std::fs;
use std::collections::VecDeque;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).expect("Unable to read file path");
    let mut crates: Vec<VecDeque<char>> = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            if line.contains('[') {
                let line: &[u8] = line.as_bytes();
                let mut count = 0;
                for i in (1..line.len()).step_by(4) {
                    if count >= crates.len() {
                        crates.push(VecDeque::new());
                    }
                    if line[i] != b' ' {
                        crates[count].push_back(line[i] as char);
                    }
                    count += 1
                }
            } else if line.as_bytes()[0] == b'm' {
                let numbers: Vec<&str> = line.split_whitespace().filter(|&w| is_number(w)).collect();
                let n: usize = numbers[0].parse().expect("Unable to parse instruction");
                let source: usize = numbers[1].parse().expect("Unable to parse instructions");
                let dest: usize = numbers[2].parse().expect("Unable to parse instructions");
                let mut buffer: Vec<char> = Vec::new();
                for _ in 0..n {
                    let top_crate = crates[source - 1].pop_front().expect("Popped from an empty stack");
                    buffer.push(top_crate);
                }
                buffer.reverse();
                for n in buffer {
                    crates[dest - 1].push_front(n)
                }
            }
        }
    }
    for st in crates {
        print!("{}", st.front().unwrap_or(&' '));
    }
}

fn is_number(input: &str) -> bool {
    input.parse::<i32>().is_ok()
}