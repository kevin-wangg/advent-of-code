use std::collections::HashSet;
use std::{env, fs};

const WIDTH: i32 = 7;

#[derive(Clone, Debug)]
struct Rock {
    pos: Vec<(i32, i32)>,
}

impl Rock {
    fn new(pos: Vec<(i32, i32)>) -> Self {
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
    occupied: HashSet<(i32, i32)>,
    height: i32,
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
        }
    }

    #[allow(dead_code)]
    fn print(&self, full: bool) {
        let start = if full {
            0
        } else {
            i32::max(0, self.height - 50)
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
        if self.height < 2675 {
            return;
        }
        let mut s = String::new();
        let _ = std::io::stdin().read_line(&mut s);
    }
    fn drop_rock(&mut self) {
        // self.print();
        self.make_rock();
        while self.step() {}
        self.height = self.calculate_height();
    }

    fn calculate_height(&self) -> i32 {
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
        // println!("dir {}", dir);
        // self.print(false);
        // thread::sleep(time::Duration::from_millis(200));
        // self.wait();
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
        // self.print(false);
        // thread::sleep(time::Duration::from_millis(200));
        // self.wait();

        let moved = self.cur_rock.clone().unwrap().move_down();
        if !self.collision(&moved) {
            self.cur_rock = Some(moved);
        } else {
            self.occupied.extend(self.cur_rock.clone().unwrap().pos.iter());
            // println!("{:?}", self.occupied);
            // self.cur_rock = None;
            // return false to indicate rock has come to rest
            return false;
        }
        // return true to keep going
        true
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist").trim().to_string();
    let rock_cnt = 2022;
    let mut cave = Cave::new(input.clone());
    for _ in 0..rock_cnt {
        cave.drop_rock();
    }
    // cave.print(true);
    println!("ans {}", cave.height);
}
