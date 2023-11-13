use std::fs;
use std::env;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() == 1 { "input.txt" } else { &args[1] };
    let input = fs::read_to_string(file_path).expect("Unable to read file");
    let input: Vec<char> = input.chars().collect();
    let marker_length = 14;
    let mut prev_chars: Vec<char> = Vec::new();
    for i in 0..marker_length - 1 {
        prev_chars.push(input[i]);
    }
    let mut ans = 0;
    for r in marker_length - 1..input.len() {
        prev_chars.push(input[r]);
        let set: HashSet<char> = prev_chars.iter().cloned().collect();
        if set.len() == marker_length {
            ans = r + 1;
            break;
        }
        prev_chars.remove(0); 
    }
    println!("{}", ans);
}
