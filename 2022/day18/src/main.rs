use std::{env, fs};
use std::collections::{HashSet, VecDeque};

const MAX_COORD: i32 = 25;

fn bfs(vis: &mut HashSet<Vec<i32>>, cubes: &HashSet<Vec<i32>>) {

    let cur = vec![MAX_COORD; 3];
    vis.insert(cur.clone());
    let mut q: VecDeque<Vec<i32>> = VecDeque::new();
    q.push_front(cur);

    while let Some(top) = q.pop_back() {
        for i in 0..3 {
            let mut v = top.clone();
            v[i] += 1;
            if v[i] <= MAX_COORD && v[i] >= -MAX_COORD && !cubes.contains(&v) && !vis.contains(&v) {
                vis.insert(v.clone());
                q.push_front(v.clone());
            }
            v[i] -= 2;
            if v[i] <= MAX_COORD && v[i] >= -MAX_COORD && !cubes.contains(&v) && !vis.contains(&v) {
                vis.insert(v.clone());
                q.push_front(v.clone());
            }
        }
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let cubes: HashSet<Vec<i32>> = input.lines().map(|l| {
        l.split(',').map(|n| n.parse::<i32>().unwrap()).collect()
    }).collect();
    let mut ans = 0;
    for c in &cubes {
        let mut exposed = 6;
        for i in 0..3 {
            let mut v1 = c.clone();
            v1[i] += 1;
            if cubes.contains(&v1) {
                exposed -= 1;
            }
            v1[i] -= 2;
            if cubes.contains(&v1) {
                exposed -= 1;
            }
        }
        ans += exposed;
    }

    let mut vis: HashSet<Vec<i32>> = HashSet::new();
    bfs(&mut vis, &cubes);

    for i in -MAX_COORD..=MAX_COORD {
        for j in -MAX_COORD..=MAX_COORD {
            for k in -MAX_COORD..=MAX_COORD {
                let v = vec![i, j, k];
                if !cubes.contains(&v) && !vis.contains(&v) {
                    for l in 0..3 {
                        let mut v = v.clone();
                        v[l] += 1;
                        if cubes.contains(&v) {
                            ans -= 1;
                        }
                        v[l] -= 2;
                        if cubes.contains(&v) {
                            ans -= 1;
                        }
                    }
                }
            }
        }
    }

    println!("ans {}", ans);
}
