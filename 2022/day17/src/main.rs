use std::{env, fs};

struct Cave {
    cave: Vec<Vec<char>>,
    height: i32
}

fn print_cave(cave: &Cave) {
    println!("Hello");
    for i in 0..cave.cave.len() {
        for j in 0..cave.cave[i].len() {
            print!("{}", cave.cave[i][j]);
        }
        println!();
    }
    println!();
}

fn process(cave: &mut Cave, moves: Vec<char>) {
    let mut rock = 0;
    let mut c_ind = 0;

    let check_valid = |cave: Vec<Vec<char>>, coords: &Vec<(usize, usize)>| {
        for c in coords {
            if c.0 >= 0 && c.0 < cave.len() && c.1 >= 0 && c.1 <= 7 {
                continue;
            } 
            return false;
        }
        return true;
    };

    while c_ind < moves.len() && c_ind < 10 {
        if cave.cave.len() as i32 - cave.height < 10 {
            cave.cave.append(&mut vec![vec!['.'; 7]; 10]);
        }
        assert!(rock <= 4);
        match rock {
            0 => {
                // horizontal rock
                // y, x is the left most point of the rock
                let mut y = (cave.height + 4) as usize;
                let mut x = 2 as usize;

                'outer: loop {
                    if moves[c_ind] == '<' {
                        // move left
                        // check if moving left doesn't hit wall and doesn't hit rock
                        if x > 0 && cave.cave[y][x - 1] == '.' {
                            x -= 1;
                        }
                    } else {
                        // move right
                        if x < 3 && cave.cave[y][x + 4] == '.' {
                            x += 1;
                        }
                    }

                    // Check if rock can move down
                    for i in 0..4 {
                        if y == 0 || cave.cave[y - 1][x + i] != '.' {
                            break 'outer;
                        }
                    }
                    c_ind += 1;
                    y -= 1;
                }

                // Draw rock in resting position
                for i in 0..4 {
                    cave.cave[y][x + i] = '#';
                }
                cave.height = y as i32;
            },
            1 => {
                // cross shaped rock
                // y, x is the middle of the rock
                let y = (cave.height + 5) as usize;
                let x = 3 as usize;

                let mut coords = vec![
                    (y, x), (y + 1, x), (y, x - 1), (y, x + 1), (y - 1, x)
                ];

                loop {
                    if moves[c_ind] == '<' {
                        let moved_coords = coords.iter().map(
                            |c| {
                                (c.0, c.1 - 1)
                            }
                        ).collect();
                        if check_valid(cave.cave.clone(), &moved_coords) {
                            coords = moved_coords;
                        }
                    } else {
                        let moved_coords = coords.iter().map(
                            |c| {
                                (c.0, c.1 + 1)
                            }
                        ).collect();
                        if check_valid(cave.cave.clone(), &moved_coords) {
                            coords = moved_coords;
                        }
                    }

                    let moved_coords = coords.iter()
                        .map(|c| {
                            (c.0 - 1, c.1)
                        })
                        .collect();

                    if check_valid(cave.cave.clone(), &moved_coords) {
                        c_ind += 1;
                        coords = moved_coords;
                    } else {
                        break;
                    }
                }

                cave.cave[y][x] = '#';
                cave.cave[y + 1][x] = '#';
                cave.cave[y][x - 1] = '#';
                cave.cave[y][x + 1] = '#';
                cave.cave[y - 1][x] = '#';

                cave.height = (y + 1) as i32;
            },
            2 => {
                // backwards L shaped rock
                // y, x is the corner of the rock
                let y = (cave.height + 4) as usize;
                let x = 4;
                cave.cave[y][x] = '#';
                cave.cave[y + 1][x] = '#';
                cave.cave[y + 2][x] = '#';
                cave.cave[y][x - 2] = '#';
                cave.cave[y][x - 1] = '#';

                cave.height = (y + 2) as i32;
            },
            3 => {
                // straight line rock
                // y, x is the bottom of the rock
                let y = (cave.height + 4) as usize;
                let x = 2;
                for i in 0..4 {
                    cave.cave[y + i][x] = '#';
                }
                cave.height = (y + 3) as i32;
            },
            4 => {
                // square rock
                // y, x is the bottom left corner of the rock
                let y = (cave.height + 4) as usize;
                let x = 2;
                cave.cave[y][x] = '#';
                cave.cave[y][x + 1] = '#';
                cave.cave[y + 1][x] = '#';
                cave.cave[y + 1][x + 1] = '#';
            
                cave.height = (y + 1) as i32;
            },
            _ => {
                panic!("Invalid rock type");
            }
        }
        rock = (rock + 1) % 5;
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".into());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut cave = Cave {
        cave: vec![vec!['.'; 7]; 10],
        height: -1,
    };
    process(&mut cave, input.chars().collect());
    print_cave(&cave);
}
