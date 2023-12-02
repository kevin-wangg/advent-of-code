use std::{env, fs};

const WIDTH: usize = 7;

struct Cave {
    cave: Vec<Vec<char>>,
    height: i32
}

fn print_cave(cave: &Cave) {
    println!("Hello");
    let start = if cave.cave.len() < 20 {
        0
    } else {
        cave.cave.len() - 20
    };
    for i in (start..cave.cave.len()).rev() {
        print!("{} ", i);
        for j in 0..cave.cave[i].len() {
            print!("{}", cave.cave[i][j]);
        }
        println!();
    }
    println!();
}

fn check_valid(grid: &Vec<Vec<char>>, coords: &Vec<(i32, i32)>) -> bool {
    for c in coords {
        if c.0 >= 0 && c.1 >= 0 {
            let y = c.0 as usize;
            let x = c.1 as usize;
            if y < grid.len() && x < WIDTH {
                if grid[y][x] != '.' {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}

fn draw_rock(grid: &mut Vec<Vec<char>>, coord: &Vec<(i32, i32)>) {
    for c in coord {
        let y = c.0 as usize;
        let x = c.1 as usize;
        grid[y][x] = '#';
    }
}

fn do_moves(moves: Vec<char>) -> Cave {
    let mut cave = Cave {
        cave: vec![vec!['.'; WIDTH]; 10],
        height: -1,
    };
    let mut rock = 0;
    let mut rock_cnt = 0;
    let mut c_ind = 0;

    while rock_cnt < 2022 {
        println!("{}", rock_cnt);
        if cave.cave.len() as i32 - cave.height < 10 {
            cave.cave.append(&mut vec![vec!['.'; 7]; 10]);
        }
        assert!(rock <= 4);
        let (mut coords, mut height) = match rock {
            0 => {
                // Horizontal rock
                // y, x is the left most point of the rock
                let y = cave.height + 4;
                let x = 2;
                let coords = vec![
                    (y, x), (y, x + 1), (y, x + 2), (y, x + 3)
                ];
                (coords, y)
            },
            1 => {
                // Cross shape rock
                // y, x is the center of the rock
                let y = cave.height + 5;
                let x = 3;
                let coords = vec![
                    (y, x), (y + 1, x), (y, x - 1), (y, x + 1), (y - 1, x)
                ];
                (coords, y + 1)
            },
            2 => {
                // Backwards L shape rock
                // y, x is the corner of the rock
                let y = cave.height + 4;
                let x = 4;
                let coords = vec![
                    (y, x), (y + 1, x), (y + 2, x), (y, x - 2), (y, x - 1)
                ];
                (coords, y + 2)
            },
            3 => {
                // Vertical rock
                // y, x is the bottom of the rock
                let y = cave.height + 4;
                let x = 2;
                let coords = vec![
                    (y, x), (y + 1, x), (y + 2, x), (y + 3, x)
                ];
                (coords, y + 3)
            },
            4 => {
                // Square rock
                // y, x is the bottom left corner
                let y = cave.height + 4;
                let x = 2;
                let coords = vec![
                    (y, x), (y, x + 1), (y + 1, x), (y + 1, x + 1)
                ];
                (coords, y + 1)
            },
            _ => panic!("Invalid rock type")
        };

        loop {
            println!("{:?} {}", coords, moves[c_ind]);
            if moves[c_ind] == '<' {
                // move left
                let moved_coords = coords.iter().map(
                    |c| {
                        (c.0, c.1 - 1)
                    }
                ).collect();
                if check_valid(&cave.cave, &moved_coords) {
                    coords = moved_coords;
                }
            } else {
                // move right
                let moved_coords = coords.iter().map(
                    |c| {
                        (c.0, c.1 + 1)
                    }
                ).collect();
                if check_valid(&cave.cave, &moved_coords) {
                    coords = moved_coords;
                }
            }
            c_ind += 1;
            if c_ind == moves.len() {
                c_ind = 0;
            }
            // Check if rock can move down
            let moved_coords = coords.iter().map(
                |c| {
                    (c.0 - 1, c.1)
                }
            ).collect();
            if check_valid(&cave.cave, &moved_coords) {
                coords = moved_coords;
                height -= 1;
            } else {
                break;
            }
            // print_cave(&cave);
            // let mut s = String::from("");
            // std::io::stdin().read_line(&mut s);
        }

        cave.height = i32::max(cave.height, height);
        rock = (rock + 1) % 5;
        rock_cnt += 1;
        draw_rock(&mut cave.cave, &coords);
        // print_cave(&cave);
        // println!("{}", cave.height);
        // let mut s = String::from("");
        // std::io::stdin().read_line(&mut s);
    }
    cave
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".into());
    let input = fs::read_to_string(path).expect("File should exist");
    let cave = do_moves( input.chars().collect());
    println!("ans {}", cave.height + 1);
    let mut height = 0;
    print_cave(&cave);
    for (h, row) in cave.cave.iter().enumerate().rev() {
        if row.contains(&'#') {
            height = h;
            break;
        }
    }
    println!("{height}");
}
