#![allow(dead_code, unused)]

use std::{fs::read_to_string, i32, num, u32};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn day4_p1() -> u32 {
    let input = read_lines("src/day4/input.txt");
    let mut res: u32 = 0;

    for line in input {
        let mut count: u32 = 0;

        let mut left_section = &line[line.find(":").unwrap() + 1..line.find("|").unwrap()].trim();
        let mut right_section = &line[line.find("|").unwrap() + 1..].trim();

        let mut left_vec: Vec<i32> = left_section
            .split_ascii_whitespace()
            .map(|n| -> i32 { n.parse().unwrap() })
            .collect();

        let mut right_vec: Vec<i32> = right_section
            .split_ascii_whitespace()
            .map(|n| -> i32 { n.parse().unwrap() })
            .collect();

        left_vec.into_iter().for_each(|n| {
            if right_vec.contains(&n) {
                count += 1;
            }
        });

        res += match count {
            0..=1 => count,
            _ => (2_i32).pow(count - 1) as u32,
        };
    }
    res
}

#[derive(Debug)]
struct CardInfo {
    game_number: i32,
    card_count: i32,
}

fn day4_p2() -> i32 {
    let mut input = read_lines("src/day4/input2.txt");
    let mut res: i32 = 0;
    let cards_count = input.len();

    let mut cards: Vec<CardInfo> = (1..=cards_count)
        .map(|i| CardInfo {
            game_number: i as i32,
            card_count: 1,
        })
        .collect();

    for (i, line) in input.into_iter().enumerate() {
        let mut count: usize = 0;
        let game_id = i + 1;

        let mut left_section = &line[line.find(":").unwrap() + 1..line.find("|").unwrap()].trim();
        let mut right_section = &line[line.find("|").unwrap() + 1..].trim();

        let mut left_vec: Vec<i32> = left_section
            .split_ascii_whitespace()
            .map(|n| -> i32 { n.parse().unwrap() })
            .collect();

        let mut right_vec: Vec<i32> = right_section
            .split_ascii_whitespace()
            .map(|n| -> i32 { n.parse().unwrap() })
            .collect();

        left_vec.into_iter().for_each(|n| {
            if right_vec.contains(&n) {
                count += 1;
            }
        });

        for _ in 0..cards[i].card_count {
            for c in game_id..game_id + count {
                cards[c].card_count += 1;
            }
        }
    }

    res = cards.into_iter().map(|c| c.card_count).sum();

    res
}

pub fn day4() {
    println!("{}", day4_p1());
    println!("{}", day4_p2());
}
