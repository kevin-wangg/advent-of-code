use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = if args.len() == 1 {
        "input.txt"
    } else {
        &args[1]
    };
    let input = fs::read_to_string(filepath).expect("Unable to read file");
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    let height = 6;
    let width = 40;
    let mut crt: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    for line in input.lines() {
        let instr: Vec<&str> = line.split_whitespace().collect(); 
        draw_pixel(&cycle, &x, &mut crt);
        if instr.len() == 1 {
            // noop
        } else {
            // addx
            let v: i32 = instr[1].parse().expect("Unable to parse V");
            cycle += 1;
            draw_pixel(&cycle, &x, &mut crt);
            x += v;
        }
        cycle += 1;
    }
    for i in 0..6 {
        for j in 0..40 {
            print!("{}", crt[i][j]);
        }
        println!();
    }
}

fn draw_pixel(cycle: &i32, sprite: &i32, crt: &mut Vec<Vec<char>>) {
    let y = (cycle - 1) / 40;
    let x: i32 = (cycle - 1) % 40;
    if *sprite == x || sprite - 1 == x || sprite + 1 == x {
        crt[y as usize][x as usize] = '#';
    }
}