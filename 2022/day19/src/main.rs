use std::collections::HashMap;
use std::collections::VecDeque;
use std::{env, fs};

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost: (i32, i32),
    geode_robot_cost: (i32, i32),
}

impl Blueprint {
    fn new(
        id: i32,
        ore_robot_cost: i32,
        clay_robot_cost: i32,
        obsidian_robot_cost: (i32, i32),
        geode_robot_cost: (i32, i32),
    ) -> Self {
        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    ore_amt: i32,
    clay_amt: i32,
    obs_amt: i32,
    ore_rbt: i32,
    clay_rbt: i32,
    obs_rbt: i32,
    geo_rbt: i32,
}

impl State {
    fn start() -> Self {
        Self {
            ore_amt: 0,
            clay_amt: 0,
            obs_amt: 0,
            ore_rbt: 1,
            clay_rbt: 0,
            obs_rbt: 0,
            geo_rbt: 0,
        }
    }
}

fn rec(bp: &Blueprint, seen: &mut HashMap<State, i32>, s: State) -> i32 {
    if seen.contains_key(&s) {
        return seen[&s];
    }
    let mut options = vec![];
    if s.ore_amt >= bp.ore_robot_cost {
        options.push("ore");
    }
    if s.clay_amt >= bp.clay_robot_cost {
        options.push("clay");
    }
    if s.ore_amt >= bp.obsidian_robot_cost.0 && s.clay_amt >= bp.obsidian_robot_cost.1 {
        options.push("obs");
    }
    if s.ore_amt >= bp.geode_robot_cost.0 && s.obs_amt >= bp.geode_robot_cost.1 {
        options.push("geode");
    }
    if options.len() < 4 {
        options.push("wait");
    }
    let mut ret = 0;
    for option in options {
        let mut news = s;
        match option {
            "ore" => {
                news.ore_amt -= bp.ore_robot_cost;
                news.ore_rbt += 1;
            }
            "clay" => {
                news.clay_amt -= bp.clay_robot_cost;
                news.clay_rbt += 1;
            }
            "obs" => {
                news.obs_amt -= bp.obsidian_robot_cost.0;
                news.clay_amt -= bp.obsidian_robot_cost.1;
                news.obs_rbt += 1;
            }
            "geode" => {
                news.ore_amt -= bp.geode_robot_cost.0;
                news.obs_amt -= bp.geode_robot_cost.1;
                news.geo_rbt += 1;
            }
            "wait" => {
                // don't make more robots
            }
            _ => {
                panic!("Invalid option");
            }
        }

        // Robots collect material
        news.ore_amt += s.ore_rbt;
        news.clay_amt += s.clay_rbt;
        news.obs_amt += s.obs_rbt;

        ret = ret.max(rec(bp, seen, news));
    }
    seen.insert(s, ret);
    ret
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in input.lines() {
        let line = line.split(": ").collect::<Vec<&str>>();
        let id = line[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let nums = line[1]
            .split_whitespace()
            .filter(|w| w.parse::<i32>().is_ok())
            .map(|w| w.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        blueprints.push(Blueprint::new(
            id,
            nums[0],
            nums[1],
            (nums[2], nums[3]),
            (nums[4], nums[5]),
        ));
    }

    let initial_state = State::start();
    let mut seen = HashMap::new();
    for bp in blueprints {
        let ans = rec(&bp, &mut seen, initial_state);
        println!("max {}", ans);
    }
}
