use std::collections::{HashMap, HashSet};
use std::{env, fs};

const WIDTH: i64 = 7;
const NUM_ROCKS: i64 = 1_000_000_000_000;

#[derive(Clone, Debug)]
struct Rock {
    pos: Vec<(i64, i64)>,
}

impl Rock {
    fn new(pos: Vec<(i64, i64)>) -> Self {
        Rock { pos }
    }

    fn move_down(&self) -> Self {
        Rock {
            pos: self.pos.iter().map(|&(x, y)| (x, y - 1)).collect(),
        }
    }

    fn move_left(&self) -> Self {
        Rock {
            pos: self.pos.iter().map(|&(x, y)| (x - 1, y)).collect(),
        }
    }

    fn move_right(&self) -> Self {
        Rock {
            pos: self.pos.iter().map(|&(x, y)| (x + 1, y)).collect(),
        }
    }
}

struct Cave {
    rock_ind: usize,
    cur_rock: Option<Rock>,
    jets_ind: usize,
    jets: Vec<char>,
    occupied: HashSet<(i64, i64)>,
    height: i64,
    seen: HashMap<(Vec<i64>, usize, usize), (i64, i64)>,
    top_n: i64,
    offset: i64,
}

impl Cave {
    fn new(jets: String) -> Self {
        Cave {
            rock_ind: 0,
            cur_rock: None,
            jets_ind: 0,
            jets: jets.chars().collect(),
            occupied: HashSet::new(),
            height: 0,
            seen: HashMap::new(),
            top_n: 30,
            offset: 0,
        }
    }

    #[allow(dead_code)]
    fn print(&self, full: bool) {
        let start = if full {
            0
        } else {
            i64::max(0, self.height - 60)
        };

        for i in (start..=self.height + 10).rev() {
            print!("{}\t", i);
            if i == 0 {
                print!("+");
            } else {
                print!("|");
            }
            for j in 0..WIDTH {
                if i == 0 {
                    print!("-");
                    continue;
                }
                if let Some(ref rock) = self.cur_rock {
                    if rock.pos.contains(&(j, i)) {
                        print!("@");
                        continue;
                    }
                }
                if self.occupied.contains(&(j, i)){
                    print!("#");
                } else {
                    print!(".");
                }
            }
            if i == 0 {
                print!("+");
            } else {
                print!("|");
            }
            println!();
        }
    }

    fn make_rock(&mut self) {
        let x = 2;
        let y = self.height + 4;
        let pos = vec![
            // horizontal line
            vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            // plus
            vec![
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
                (x + 1, y),
            ],
            // backwards L
            vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            // vertical line
            vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            // square shape
            vec![(x, y), (x + 1, y), (x + 1, y + 1), (x, y + 1)],
        ];
        self.cur_rock = Some(Rock::new(pos[self.rock_ind].clone()));
        self.rock_ind = (self.rock_ind + 1) % 5;
    }

    fn get_dir(&mut self) -> char {
        let c = self.jets[self.jets_ind];
        self.jets_ind = (self.jets_ind + 1) % self.jets.len();
        c
    }

    #[allow(dead_code)]
    fn wait(&self) {
        let mut s = String::new();
        let _ = std::io::stdin().read_line(&mut s);
    }

    fn serialize_top_n_rows(&self) -> Vec<i64> {
        if self.height < self.top_n {
            panic!("Can't serialize top {}", self.top_n);
        }
        let mut ret = Vec::with_capacity(self.top_n as usize);
        for i in self.height - self.top_n..=self.height {
            let mut n = 0;
            let mut c = 1;
            for j in (0..7).rev() {
                if self.occupied.contains(&(j, i)) {
                    n += c;
                }
                c *= 2;
            }
            ret.push(n);
        }
        ret
    }

    fn drop_rock(&mut self, rock_cnt: &mut i64) {
        self.make_rock();
        while self.step() {}
        self.height = self.calculate_height();
        if self.height >= self.top_n {
            // Store top N rows, jets_ind, and rocks_ind with height
            let key = (self.serialize_top_n_rows(), self.jets_ind, self.rock_ind);
            if self.seen.contains_key(&key) {
                let past_height = self.seen[&key].1;
                let past_rock = self.seen[&key].0;
                let remaining = NUM_ROCKS - *rock_cnt;
                let diff = *rock_cnt - past_rock;
                let times = remaining / diff;
                if times > 0 {
                    *rock_cnt += diff * times;
                    self.offset = (self.height - past_height) * times; 
                    return;
                }
            } else {
                self.seen.insert(key, (*rock_cnt, self.height));
            } 
        }
        *rock_cnt += 1;
    }

    fn calculate_height(&self) -> i64 {
        self.occupied.iter().fold(0, |acc, &(_, y)| acc.max(y))
    }

    fn collision(&self, rock: &Rock) -> bool {
        for p in &rock.pos {
            if p.0 < 0 || p.0 >= WIDTH || p.1 <= 0 {
                // out of bounds
                return true;
            }
            if self.occupied.contains(p) {
                // hit other rock
                return true;
            }
        }
        return false;
    }

    fn step(&mut self) -> bool {
        let dir = self.get_dir();
        if dir == '<' {
            let moved = self.cur_rock.clone().unwrap().move_left();
            if !self.collision(&moved) {
                self.cur_rock = Some(moved);
            }
        } else if dir == '>' {
            let moved = self.cur_rock.clone().unwrap().move_right();
            if !self.collision(&moved) {
                self.cur_rock = Some(moved);
            }
        } else {
            panic!("Invalid direction");
        }
        let moved = self.cur_rock.clone().unwrap().move_down();
        if !self.collision(&moved) {
            self.cur_rock = Some(moved);
        } else {
            self.occupied.extend(self.cur_rock.clone().unwrap().pos.iter());
            self.cur_rock = None;
            return false;
        }
        // return true to keep going
        true
    }
}

fn solve(input: &str) {
    // let rock_cnt = NUM_ROCKS;
    let mut cave = Cave::new(input.to_string());
    let mut r = 0;
    while r < NUM_ROCKS {
        cave.drop_rock(&mut r);
    }
    println!("ans {}", cave.height + cave.offset - 1);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist").trim().to_string();
    solve(&input);
}
