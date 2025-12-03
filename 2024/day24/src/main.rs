use std::collections::HashMap;
use std::env;
use std::fs;

enum Connection {
    OR,
    XOR,
    AND,
}

struct Gate {
    connection: Connection,
    a: String,
    b: String,
}

fn get_value(cur: &str, gates: &HashMap<String, Gate>, values: &mut HashMap<String, bool>) -> bool {
    println!("cur {}", cur);
    if values.contains_key(cur) {
        values[cur]
    } else {
        let v1 = get_value(&gates[cur].a, gates, values);
        let v2 = get_value(&gates[cur].b, gates, values);
        let value = match gates[cur].connection {
            Connection::OR => v1 || v2,
            Connection::AND => v1 && v2,
            Connection::XOR => v1 ^ v2,
        };
        values.insert(cur.to_string(), value);
        value
    }
}

fn solve(input: &str) {
    let mut values: HashMap<String, bool> = HashMap::new();
    let mut gates: HashMap<String, Gate> = HashMap::new();
    let input: Vec<_> = input.split("\n\n").collect();

    for line in input[0].lines() {
        let line: Vec<_> = line.split(": ").collect();
        let value = if line[1] == "1" { true } else { false };
        values.insert(line[0].to_string(), value);
    }

    let mut ends = Vec::new();

    for line in input[1].lines() {
        let line: Vec<_> = line.split(" -> ").collect();
        let out = line[1];
        let inputs: Vec<_> = line[0].split_whitespace().collect();
        let connection = match inputs[1] {
            "AND" => Connection::AND,
            "OR" => Connection::OR,
            "XOR" => Connection::XOR,
            op => {
                panic!("Invalid operator {}", op);
            }
        };
        gates.insert(
            out.to_string(),
            Gate {
                connection,
                a: inputs[0].to_string(),
                b: inputs[2].to_string(),
            },
        );

        ends.push(out);
        ends.retain(|&v| v != inputs[0] && v != inputs[1]);
    }

    for e in ends {
        get_value(e, &gates, &mut values);
    }
    let mut keys = Vec::new();
    for (k, v) in values {
        if k.starts_with('z') {
            keys.push((k, v));
        }
    }
    keys.sort();
    keys.reverse();
    let mut ans = String::new();
    for (_k, v) in keys {
        if v {
            ans.push('1');
        } else {
            ans.push('0');
        }
    }
    let ans = u64::from_str_radix(&ans, 2).unwrap(); 

    println!("{}", ans);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
