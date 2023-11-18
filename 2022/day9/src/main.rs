use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    y: i32,
    x: i32,
}

impl Pos {
    fn move_up(&mut self) {
        self.y += 1;        
    }
    fn move_down(&mut self) {
        self.y -= 1;        
    }
    fn move_left(&mut self) {
        self.x -= 1;        
    }
    fn move_right(&mut self) {
        self.x += 1;        
    }

    fn diff(a: &Pos, b: &Pos) -> Pos {
        Pos {
            y: a.y - b.y,
            x: a.x - b.x
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = if args.len() == 1 {
        "input.txt"
    } else {
        &args[1]
    };
    let input = fs::read_to_string(filepath)
        .expect("Unable to read file");
    let mut tail_positions: HashSet<Pos> = HashSet::new();
    let mut knots: Vec<Pos> = Vec::new();
    for _ in 0..10 {
        knots.push(Pos { y: 0, x: 0 });
    }
    tail_positions.insert(knots[0]);
    for line in input.lines() {
        let motion: Vec<&str> = line.split_whitespace().collect();
        let dir = motion[0];
        let steps: usize = motion[1].parse().unwrap();
        for _ in 0..steps {
            move_head(dir, &mut knots[0]);
            for i in 0..9 {
                process(
                    &mut knots,
                    i,
                    i + 1
                )
            }
            tail_positions.insert(knots[9]);
            // print_rope(&knots);
        }
    }
    println!("{}", tail_positions.len());
}

fn move_head(dir: &str, head_pos: &mut Pos) {
    match dir {
        "U" => {
            head_pos.move_up();
        },
        "D" => {
            head_pos.move_down();
        },
        "L" => {
            head_pos.move_left();
        },
        "R" => {
            head_pos.move_right();
        },
        _ => {
            panic!("Unknown direction")
        }
    }
}

fn process(
    knots: &mut Vec<Pos>,
    i: usize,
    j: usize,
) {
    let d = Pos::diff(&knots[i], &knots[j]);
    if d.x == 0 {
        if d.y == 2 {
            knots[j].move_up();
        } else if d.y == -2 {
            knots[j].move_down();
        }
    } else if d.y == 0 {
        if d.x == 2 {
            knots[j].move_right();
        } else if d.x == -2 {
            knots[j].move_left();
        }
    } else if d.x == 2 {
        knots[j].move_right();
        if d.y == 1 {
            knots[j].move_up();
        } else if d.y == -1 {
            knots[j].move_down();
        } else if d.y == 2 {
            knots[j].move_up();
        } else if d.y == -2 {
            knots[j].move_down();
        }
    } else if d.x == -2 {
        knots[j].move_left();
        if d.y == 1 {
            knots[j].move_up();
        } else if d.y == -1 {
            knots[j].move_down();
        } else if d.y == 2 {
            knots[j].move_up();
        } else if d.y == -2 {
            knots[j].move_down();
        }
    } else if d.y == 2 {
        knots[j].move_up();
        if d.x == 1 {
            knots[j].move_right();
        } else if d.x == -1 {
            knots[j].move_left();
        } else if d.x == 2 {
            knots[j].move_right();
        } else if d.x == -2 {
            knots[j].move_left();
        }
    } else if d.y == -2 {
        knots[j].move_down();
        if d.x == 1 {
            knots[j].move_right();
        } else if d.x == -1 {
            knots[j].move_left();
        } else if d.x == 2 {
            knots[j].move_right();
        } else if d.x == -2 {
            knots[j].move_left();
        }
    }
}

fn print_rope(knots: &Vec<Pos>) {
    let rows = 5;
    let cols = 6;
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    for i in (0..10).rev() {
        let c = if i == 0 {
            'H'
        } else {
            char::from_digit(i as u32, 10).unwrap()
        };
        grid[knots[i].y as usize][knots[i].x as usize] = c;
    }
    for i in 0..rows {
        for j in 0..cols {
            print!("{}", grid[i][j]);
        }
        println!();
    }
    println!();
}