use std::io;

trait Scorable {
    fn score(&self) -> i32;
}

enum Outcome {
    Win,
    Loss,
    Draw
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

use crate::Outcome::{Win, Loss, Draw};
use crate::Move::{Rock, Paper, Scissors};

impl Scorable for Outcome {
    fn score(&self) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

impl Scorable for Move {
    fn score(&self) -> i32 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}


fn get_move(s: &str) -> Move {
    if s == "A" {
        Rock
    } else if s == "B" {
        Paper
    } else if s == "C" {
        Scissors
    } else {
        panic!("Invalid move");
    }
}

fn get_outcome(s: &str) -> Outcome {
    match s {
        "X" => Loss,
        "Y" => Draw,
        "Z" => Win,
        m => panic!("Invalid outcome: {}", m)
    }
}

fn my_move(m: &Move, o: &Outcome) -> Move {
    match m {
        Rock => {
            match o {
                Win => Paper,
                Draw => Rock,
                Loss => Scissors,
            }
        },
        Paper => {
            match o {
                Win => Scissors,
                Draw => Paper,
                Loss => Rock,
            }
        },
        Scissors => {
            match o {
                Win => Rock,
                Draw => Scissors,
                Loss => Paper,
            }
        },
    }
}

fn main() {
    let mut input = String::new();
    let mut ans = 0;
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let mut it = input.split_whitespace();
                let m = get_move(it.next().unwrap());
                let outcome = get_outcome(it.next().unwrap());
                let cur = my_move(&m, &outcome);
                ans += outcome.score();
                ans += cur.score();
            }
            Err(_) => {
                eprintln!("Failed to read input!")
            }
        }
        input.clear();
    }
    println!("{ans}");
}
