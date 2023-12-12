#![allow(dead_code, unused)]

use std::{fs::read_to_string, i32, num, u32};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Debug)]
struct Game<T> {
    time: T,
    distance: T,
}

fn day6_p1() -> u32 {
    let input = read_lines("src/day6/input.txt");
    let mut times_beat: Vec<u32> = Vec::new();
    let mut games = Vec::new();
    let times = input[0].split_ascii_whitespace().skip(1);
    let distances = input[1].split_ascii_whitespace().skip(1);

    for (time, distance) in times.zip(distances) {
        games.push(Game {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        });
    }

    for game in games {
        let mut beat = 0;

        for t in 1..game.time {
            let time_to_travel = game.time - t;
            let distance_traveled = time_to_travel * t;
            if distance_traveled > game.distance {
                beat += 1;
            }
        }

        times_beat.push(beat);
    }

    times_beat.iter().fold(1, |acc, &n| acc * n)
}

/// Absolutely horible solution
fn day6_p2() -> i32 {
    let input = read_lines("src/day6/input.txt");
    let mut times_beat: Vec<u32> = Vec::new();
    let t: i64 = input[0][input[0].find(":").unwrap() + 1..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: i64 = input[1][input[1].find(":").unwrap() + 1..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let game = Game { distance, time: t };
    let mut beat = 0;

    for t in 1..game.time {
        let time_to_travel = game.time - t;
        let distance_traveled = time_to_travel * t;
        if distance_traveled > game.distance {
            beat += 1;
        }
    }
    beat
}

pub fn day6() {
    // println!("{}", day6_p1());
    println!("{}", day6_p2());
}
