use std::{env, fs};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourKind(Vec<Card>),
    FiveKind(Vec<Card>),
}

fn n_kind(hand: &str, n: u32) -> bool {
    let hand: Vec<char> = hand.chars().collect();
    let mut occ: HashMap<char, u32> = HashMap::new();
    let mut j_count = 0;
    for c in &hand {
        occ.entry(*c).and_modify(|v| *v += 1).or_insert(1);
        if *c == 'J' {
            j_count += 1;
        }
    }
    for (k, v) in occ {
        if v == n {
            return true;
        }
        if k != 'J' && v + j_count == n {
            // Can use j_count to reach n if needed
            return true; 
        }
    }
    false
}

fn full_house(hand: &str) -> bool {
    let hand: Vec<char> = hand.chars().filter(|&c| c != 'J').collect();
    let mut occ: HashSet<char> = HashSet::new();
    for c in &hand {
        occ.insert(*c);
    }
    occ.len() == 2
}

fn two_pair(hand: &str) -> bool {
    let hand: Vec<char> = hand.chars().filter(|&c| c != 'J').collect();
    let mut occ: HashSet<char> = HashSet::new();
    for c in &hand {
        occ.insert(*c);
    }
    occ.len() == 3
}

use crate::Card::*;
use crate::Hand::*;

fn to_cards(hand: &str) -> Vec<Card> {
    hand.chars().map(|c| {
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Joker,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!("Invalid card type")
        }

    }).collect()
}

fn get_hand(hand: &str) -> Hand {
    if n_kind(hand, 5) {
        FiveKind(to_cards(hand))
    } else if n_kind(hand, 4) {
        FourKind(to_cards(hand))
    } else if full_house(hand) {
        FullHouse(to_cards(hand))
    } else if n_kind(hand, 3) {
        ThreeKind(to_cards(hand))
    } else if two_pair(hand) {
        TwoPair(to_cards(hand))
    } else if n_kind(hand, 2) {
        OnePair(to_cards(hand))
    } else {
        HighCard(to_cards(hand))
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    let mut hands: Vec<(Hand, usize)> = input.lines().map(|line| {
        let line: Vec<&str> = line.split_whitespace().collect();
        (get_hand(line[0]), line[1].parse::<usize>().unwrap())
    }).collect();
    hands.sort();
    // println!("{:?}", hands);
    let mut ans = 0;
    for i in 0..hands.len() {
        ans += (i + 1) * hands[i].1;
    }
    println!("ans {}", ans);
}
