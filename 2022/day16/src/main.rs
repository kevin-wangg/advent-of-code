use std::{env, fs};
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    // Not sure if bidirectional
    leads_to: Vec<String>
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() == 1 {
        "input.txt"
    } else {
        &args[1]
    };
    let names_re = Regex::new(r"[A-Z]{2}").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();
    let input = fs::read_to_string(path).expect("File should exist");
    let mut opened: HashMap<String, bool> = HashMap::new();
    let valves: HashMap<String, Valve> = input.lines()
        .map(|line| {
            let rate = number_re.find(line).unwrap().as_str().parse::<i32>().unwrap();
            let names: Vec<String> = names_re.find_iter(line).map(|m| m.as_str().into()).collect();
            let valve = Valve {
                flow_rate: rate,
                leads_to: names[1..].to_vec()
            };
            opened.insert(names[0].clone(), false);
            (names[0].clone(), valve)
        })
        .collect();
    let dists = compute_dists(&valves);
    let mut avail_valves: HashSet<&String> = valves.iter()
        .filter(|(_, v)| {
            v.flow_rate > 0
        })
        .map(|(s, _)| {
            s
        })
        .collect();
    let ans = rec(&valves, &mut avail_valves, &dists, "AA", 26, 0, false);
    println!("ans {}", ans);
}

fn rec(
    valves: &HashMap<String, Valve>,
    avail_valves: &mut HashSet<&String>,
    dists: &HashMap<String, HashMap<String, i32>>,
    cur: &str,
    time_left: i32,
    acc: i32,
    is_elephant: bool
) -> i32 {
    // println!("valves {:?} time {} acc {} cur {}", avail_valves, time_left, acc, cur);
    // println!("cur {}", cur);
    let mut ret = acc;

    if !is_elephant {
        // I stop, elephant goes
        ret = ret.max(rec(valves, avail_valves, dists, "AA", 26, acc, true));
    }
    for v in avail_valves.clone() {
        let dist = dists[cur][v];
        if time_left - dist - 1 >= 0 {
            avail_valves.remove(v);
            ret = ret.max(rec(valves, avail_valves, dists, v, time_left - dist - 1, acc + (valves[v].flow_rate * (time_left - dist - 1)), is_elephant));
            avail_valves.insert(v);
        }
    }
    ret
}

fn compute_dists(valves: &HashMap<String, Valve>) -> HashMap<String, HashMap<String, i32>> {
    let mut ret = HashMap::new();
    for (v, _) in valves {
        let mut bfs: VecDeque<String> = VecDeque::new();
        let mut dist: HashMap<String, i32> = HashMap::new();
        bfs.push_front(v.into());
        dist.entry(v.into()).or_insert(0);

        while let Some(top) = bfs.pop_back() {
            let cur_dist = dist[&top];
            for next in &valves[&top].leads_to {
                if !dist.contains_key(next) {
                    bfs.push_front(next.into());
                    dist.entry(next.into()).or_insert(cur_dist + 1);
                }
            }
        }
        ret.insert(v.into(), dist);
    }
    ret
}