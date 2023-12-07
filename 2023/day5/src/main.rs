use std::collections::VecDeque;
use std::{env, fs};

fn get_initial(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .filter_map(|w| w.parse::<i64>().ok())
        .collect()
}

fn convert(a: (i64, i64)) -> (i64, i64) {
    (a.0, a.1 - a.0 + 1)
}

fn get_sections(src: (i64, i64), target: (i64, i64)) -> Option<((i64, i64), Vec<(i64, i64)>)> {
    let sa = src.0;
    let sb = src.0 + src.1 - 1;
    let ta = target.0;
    let tb = target.0 + target.1 - 1;
    if sb < ta || tb < sa {
        // src and target are disjoint, no intersection
        None
    } else if sa >= ta && sb <= tb {
        // src is completely inside target
        Some((convert((sa, sb)), Vec::new()))
    } else if sa <= ta && sb >= tb {
        // src completely encloses target
        let intersect = (ta, tb);
        let left = (sa, ta - 1);
        let right = (tb + 1, sb);
        let mut rem = vec![];
        if left.0 <= left.1 {
            rem.push(convert(left));
        }
        if right.0 <= right.1 {
            rem.push(convert(right));
        }
        Some((convert(intersect), rem))
    } else if sa <= ta && sb < tb {
        // sa ... ta ... sb ... tb
        let intersect = (ta, sb);
        let left = (sa, ta - 1);
        let mut rem = vec![];
        if left.0 <= left.1 {
            rem.push(convert(left));
        }
        Some((convert(intersect), rem))
    } else if sa > ta && sb >= tb {
        // ta ... sa .. tb .. sb
        let intersect = (sa, tb);
        let right = (tb + 1, sb);
        let mut rem = vec![];
        if right.0 <= right.1 {
            rem.push(convert(right));
        }
        Some((convert(intersect), rem))
    } else {
        panic!("Shouldn't reach here");
    }
}

fn get_min_location(s: i64, l: i64, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut cur: VecDeque<(i64, i64)> = VecDeque::new();
    cur.push_back((s, l));
    let mut next: VecDeque<(i64, i64)> = VecDeque::new();
    for m in maps {
        'outer: while let Some(front) = cur.pop_front() {
            for r in m {
                match get_sections(front, (r[1], r[2])) {
                    Some((intersect, rem)) => {
                        for re in rem {
                            cur.push_back(re);
                        }
                        let diff = r[1] - r[0];
                        let new_sec = (intersect.0 - diff, intersect.1);
                        next.push_back(new_sec);
                        continue 'outer;
                    }
                    None => {}
                }
            }
            next.push_back(front);
        }
        cur = next.clone();
        next.clear();
    }
    let mut locs: Vec<(i64, i64)> = cur.into_iter().collect();
    locs.sort();
    locs[0].0
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let input: Vec<&str> = input.lines().collect();
    let initial = get_initial(input[0]);
    let mut maps: Vec<_> = input.split(|&l| l.is_empty()).collect();
    maps.remove(0);
    let maps: Vec<Vec<Vec<i64>>> = maps
        .iter()
        .map(|&m| {
            let m = &m[1..m.len()];
            m.iter()
                .map(|&l| {
                    l.split_whitespace()
                        .map(|c| c.parse::<i64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    let mut ans = 1e9 as i64;
    for r in initial.chunks(2) {
        ans = ans.min(get_min_location(r[0], r[1], &maps));
    }
}
