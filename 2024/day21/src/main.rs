use std::env;
use std::fs;

fn get_pos(c: char, num_pad: bool) -> (i32, i32) {
    if num_pad {
        match c {
            'A' => (3, 2),
            '0' => (3, 1),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            c => panic!("Invalid character {}", c),
        }
    } else {
        match c {
            'A' => (0, 2),
            '^' => (0, 1),
            '<' => (1, 0),
            'v' => (1, 1),
            '>' => (1, 2),
            c => panic!("Invalid character {}", c),
        }
    }
}

#[allow(dead_code)]
fn get_char(pos: (i32, i32), num_pad: bool) -> char {
    if num_pad {
        match pos {
            (3, 2) => 'A',
            (3, 1) => '0',
            (2, 0) => '1',
            (2, 1) => '2',
            (2, 2) => '3',
            (1, 0) => '4',
            (1, 1) => '5',
            (1, 2) => '6',
            (0, 0) => '7',
            (0, 1) => '8',
            (0, 2) => '9',
            pos => panic!("Invalid pos {:?}", pos),
        }
    } else {
        match pos {
            (0, 2) => 'A',
            (0, 1) => '^',
            (1, 0) => '<',
            (1, 1) => 'v',
            (1, 2) => '>',
            pos => panic!("Invalid pos {:?}", pos),
        }
    }
}

fn make_path(src: char, dest: char, num_pad: bool) -> Vec<String> {
    let start_pos = get_pos(src, num_pad);
    let end_pos = get_pos(dest, num_pad);
    let ydiff = end_pos.0 - start_pos.0;
    let xdiff = end_pos.1 - start_pos.1;
    let vchar = if ydiff > 0 { 'v' } else { '^' };
    let hchar = if xdiff > 0 { '>' } else { '<' };

    let mut ret = Vec::new();

    let mut xfirst = String::new();
    let mut yfirst = String::new();

    for _ in 0..xdiff.abs() {
        xfirst.push(hchar);
    }
    for _ in 0..ydiff.abs() {
        xfirst.push(vchar);
    }

    for _ in 0..ydiff.abs() {
        yfirst.push(vchar);
    }
    for _ in 0..xdiff.abs() {
        yfirst.push(hchar);
    }

    let same = xfirst == yfirst;

    if num_pad {
        if (end_pos.0, start_pos.1) != (3, 0) {
            ret.push(yfirst);
        }
        if !same && (start_pos.0, end_pos.1) != (3, 0) {
            ret.push(xfirst);
        }
    } else {
        if (end_pos.0, start_pos.1) != (0, 0) {
            ret.push(yfirst);
        }
        if !same && (start_pos.0, end_pos.1) != (0, 0) {
            ret.push(xfirst);
        }
    }

    ret
}

fn key_pad(sequence: &str) -> Vec<String> {
    let mut cur = 'A';

    let mut ret = vec![String::new()];

    for c in sequence.chars() {
        let mut new_ret = Vec::new();
        let possible_paths = make_path(cur, c, false);
        for old_path in ret {
            for p in &possible_paths {
                let mut new_path = old_path.to_string();
                new_path.push_str(&p);
                new_path.push('A');
                new_ret.push(new_path);
            }
        }
        cur = c;
        ret = new_ret;
    }

    ret
}

fn num_pad(sequence: &str) -> Vec<String> {
    let mut cur = 'A';
    let mut ret = vec![String::new()];

    for c in sequence.chars() {
        let mut new_ret = Vec::new();
        let possible_paths = make_path(cur, c, true);
        for old_path in ret {
            for p in &possible_paths {
                let mut new_path = old_path.to_string();
                new_path.push_str(&p);
                new_path.push('A');
                new_ret.push(new_path);
            }
        }
        cur = c;
        ret = new_ret;
    }
    ret
}

#[allow(dead_code)]
fn enter_sequence(seq: &str, num_pad: bool) -> String {
    let mut pos = if num_pad { (3, 2) } else { (0, 2) };
    let mut ret = String::new();
    for c in seq.chars() {
        match c {
            'A' => {
                ret.push(get_char(pos, num_pad));
            }
            '>' => {
                pos.1 += 1;
            }
            '<' => {
                pos.1 -= 1;
            }
            '^' => {
                pos.0 -= 1;
            }
            'v' => {
                pos.0 += 1;
            }
            c => {
                panic!("Invalid character {}", c);
            }
        };
    }
    ret
}

fn solve(input: &str) {
    let mut ans = 0;
    for code in input.lines() {
        let seq1 = num_pad(code);
        let mut min = usize::MAX;
        for s in &seq1 {
            let seq2 = key_pad(s);
            for s in &seq2 {
                let seq3 = key_pad(s);
                for s in &seq3 {
                    min = min.min(s.len());
                }
            }
        }
        let mut code = code.to_string();
        code.pop();
        let num = code.parse::<usize>().unwrap();
        ans += num * min;
    }
    println!("ans {}", ans);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
