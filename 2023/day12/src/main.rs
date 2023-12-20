use std::{env, fs};

// fn ways(a: &Vec<char>, n: &Vec<usize>, mut ai: usize, ni: usize) -> i32 {
//     // println!("ai {} ni {}", ai, ni);
//     if ai <= a.len() && ni == n.len() {
//         // println!("return 1");
//         return 1;
//     }
//     if ai >= a.len() || ni >= n.len() {
//         // println!("return 0");
//         return 0;
//     }
//     let block = n[ni];
//     let mut ok = false;
//     if ai + block <= a.len() && (ai == 0 || a[ai - 1] != '#') {
//         for i in 0..block {
//             if a[ai + i] == '.' {
//                 ai += i;
//                 break;
//             }
//             if i == block - 1 {
//                 // when we reach end of block, if either we are at end of a or next cell is not #
//                 if ai + i == a.len() - 1 || a[ai + i + 1] != '#' {
//                     ok = true;
//                 } else {
//                     ai += 1;
//                 }
//             }
//         }
//     } else {
//         // println!("return 0");
//         return 0;
//     }
//     if ok {
//         // println!("ok");
//         // set block or don't set block
//         let tmp = if ai + block == a.len() { ai + block } else { ai + block + 1 };
//         ways(a, n, tmp, ni + 1) + ways(a, n, ai + 1, ni)
//     } else {
//         // println!("not ok");
//         while ai < a.len() && a[ai] == '.' {
//             // advance ai until next '?'
//             ai += 1;
//         }
//         ways(a, n, ai, ni)
//     }
// }

fn valid(a: Vec<char>, n: &Vec<usize>) -> bool {
    let s: String = a.iter().collect(); 
    let lens: Vec<usize> = s.split('.').filter(|&s| !s.is_empty()).map(|s| s.len()).collect();
    lens == *n
}

fn ways(mut a: Vec<char>, n: &Vec<usize>, ai: usize) -> i32 {
    if ai == a.len() {
        // check if valid
        if valid(a, n) {
            1
        } else {
            0
        }
    } else {
        if a[ai] == '?' {
            let mut ret = 0;
            a[ai] = '#';
            ret += ways(a.clone(), n, ai + 1);
            a[ai] = '.';
            ret += ways(a, n, ai + 1);
            ret
        } else {
            ways(a, n, ai + 1)
        }
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut ans = 0;
    for line in input.lines() {
        // println!("{}", line);
        let line: Vec<&str> = line.split_whitespace().collect();
        let a: Vec<char> = line[0].chars().collect();
        let n: Vec<usize> = line[1].split(',').map(|d| d.parse::<usize>().unwrap()).collect();
        let ai = a.iter().position(|&c| c != '.').unwrap();
        let w = ways(a, &n, ai);
        // println!("w {}", w);
        ans += w;
    }
    println!("ans {}", ans);
}
