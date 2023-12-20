use std::{env, fs};

const MAX_BLOCK: usize = 16;

fn ways(a: &Vec<char>, n: &Vec<usize>) -> u64 {
    let mut dp = vec![vec![vec![0; MAX_BLOCK]; n.len()]; a.len()];
    let mut ret = 0;
    if a[0] != '.' {
        dp[0][0][1] = 1;
    }
    if a[0] != '#' {
        dp[0][0][0] = 1;
    }
    for i in 1..a.len()  {
        for j in 0..n.len() {
            if j > i {
                break;
            }
            if a[i] == '.' {
                dp[i][j][0] = dp[i - 1][j][0];
                if j > 0 {
                    dp[i][j][0] += dp[i - 1][j - 1][n[j - 1]];
                }
            } else {
                for k in 0..=n[j].min(i + 1) {
                    if k == 0 {
                        if a[i] == '?' {
                            dp[i][j][k] = dp[i - 1][j][0];
                            if j > 0 {
                                dp[i][j][0] += dp[i - 1][j - 1][n[j - 1]];
                            }
                        }
                    } else {
                        dp[i][j][k] = dp[i - 1][j][k - 1];
                    }
                }
            }
        }
    }
    for i in 0..a.len() {
        let mut valid = true;
        for j in i + 1..a.len() {
            if a[j] == '#' {
                valid = false;
                break;
            }
        }
        if valid {
            ret += dp[i][n.len() - 1][*n.last().unwrap()];
        }
    }
    ret
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut ans = 0;
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let a: Vec<char> = line[0].chars().collect();
        let a = [a.clone(), vec!['?'], a.clone(), vec!['?'], a.clone(), vec!['?'], a.clone(), vec!['?'], a.clone()].concat();
        let n: Vec<usize> = line[1].split(',').map(|d| d.parse::<usize>().unwrap()).collect();
        let n = [n.clone(), n.clone(), n.clone(), n.clone(), n.clone()].concat();
        let w = ways(&a, &n);
        ans += w;
    }
    println!("ans {}", ans);
}
