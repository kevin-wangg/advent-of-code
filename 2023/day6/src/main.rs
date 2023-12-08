use std::{env, fs};

fn get_nums(s: &str) -> Vec<i64> {
    let s = s.split_whitespace()
        .filter(|&w| w.parse::<i64>().is_ok())
        .collect::<String>();
    vec![s.parse::<i64>().unwrap()]
}

fn num_ways(t: f64, d: f64) -> i64 {
    let x1 = (-t + (t * t - 4.0 * d).sqrt()) / -2.0;
    let x2 = (-t - (t * t - 4.0 * d).sqrt()) / -2.0;
    let roots = (x1.min(x2), x1.max(x2));
    let iroots = (roots.0.floor() as i64 + 1, roots.1.ceil() as i64 - 1);
    iroots.1 - iroots.0 + 1
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let input: Vec<&str> = input.lines().collect();
    let times = get_nums(input[0]);
    let dists = get_nums(input[1]);
    let mut ans = 1;
    for i in 0..times.len() {
        println!("{}", num_ways(times[i] as f64, dists[i] as f64));
        ans *= num_ways(times[i] as f64, dists[i] as f64);
    }
    println!("ans {}", ans);
}
