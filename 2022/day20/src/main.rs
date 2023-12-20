use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut a: Vec<(usize, i32)> = input.lines().enumerate().map(|(i, l)| (i, l.parse::<i32>().unwrap())).collect();
    let n = a.len();
    let mut posmap = HashMap::new();
    for (index, (id, _)) in a.iter().copied().enumerate() {
        posmap.insert(id, index);
    }
    println!("Initial");
    println!("{:?}", a);
    println!("pos map {:?}", posmap);
    println!();
    for i in 0..a.len() {
        let pos = posmap[&i];
        let (_, val) = a[pos];
        println!("move val {}", val);
        if val > 0 {
            let mut curpos = pos;
            for _ in 0..val {
                let tmp = a[curpos + 1 % n];
                a[curpos + 1 % n] = a[curpos];
                a[curpos] = tmp;
                posmap.entry(a[curpos].0).and_modify(|e| *e = curpos);
                posmap.entry(a[curpos + 1 % n].0).and_modify(|e| *e = curpos + 1 % n);
                curpos = curpos + 1 % n;
            }
        } else if val < 0 {
            let mut curpos = pos;
            for _ in 0..-val {
                if curpos == 0 {
                    let front = a.remove(0);
                    let back = a.pop().unwrap();
                    a.push(front);
                    a.push(back);
                } else {
                    let next = if curpos == 0 { n - 1 } else { curpos - 1 };
                    let tmp = a[next];
                    a[curpos - 1] = a[curpos];
                    a[curpos] = tmp;
                    posmap.entry(a[curpos].0).and_modify(|e| *e = curpos);
                    posmap.entry(a[curpos - 1].0).and_modify(|e| *e = curpos - 1);
                    curpos -= 1;
                }
            }
        }
        println!("{:?}", a);
        println!("pos map {:?}", posmap);
        println!();
    }
}
