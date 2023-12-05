#![allow(dead_code, unused)]

use std::{fs::read_to_string, num};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn day2_p1() -> i32 {
    let input = read_lines("src/day2/input.txt");
    let mut res: i32 = 0;

    for mut line in input {
        let game_id = &line[line.find(" ").unwrap() + 1..line.find(":").unwrap_or(line.len())];

        let game_str = &line[line.find(":").unwrap() + 2..];
        let mut valid = 1;
        for mut game in game_str.split(";").collect::<Vec<&str>>() {
            for mut colour in game.split(",").collect::<Vec<&str>>() {
                colour = colour.trim();

                let num = colour[0..colour.find(" ").unwrap()].parse::<i32>().unwrap();
                let col = colour[colour.find(" ").unwrap()..colour.len()].trim();

                match col {
                    "red" => {
                        if (num > 12) {
                            valid = 0;
                            break;
                        }
                    }
                    "green" => {
                        if (num > 13) {
                            valid = 0;
                            break;
                        }
                    }
                    "blue" => {
                        if (num > 14) {
                            valid = 0;
                            break;
                        }
                    }
                    _ => continue,
                }
            }
        }

        if (valid == 1) {
            res += game_id.parse::<i32>().unwrap();
        }
    }
    res
}

pub fn day2_p2() -> i32 {
    let input = read_lines("src/day2/input2.txt");
    let mut res: i32 = 0;

    for mut line in input {
        let game_id = &line[line.find(" ").unwrap() + 1..line.find(":").unwrap_or(line.len())];
        let game_str = &line[line.find(":").unwrap() + 2..];

        let mut power = 0;
        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;
        for mut game in game_str.split(";").collect::<Vec<&str>>() {
            for mut colour in game.split(",").collect::<Vec<&str>>() {
                colour = colour.trim();

                let num = colour[0..colour.find(" ").unwrap()].parse::<i32>().unwrap();
                let col = colour[colour.find(" ").unwrap()..colour.len()].trim();

                match col {
                    "red" => {
                        if (num > red_max) {
                            red_max = num;
                        }
                    }
                    "green" => {
                        if (num > green_max) {
                            green_max = num;
                        }
                    }
                    "blue" => {
                        if (num > blue_max) {
                            blue_max = num;
                        }
                    }
                    _ => continue,
                }
            }
        }
        power = red_max * blue_max * green_max;
        res += power;
    }

    res
}

pub fn day2() {
    println!("{}", day2_p1());
    println!("{}", day2_p2());
}
