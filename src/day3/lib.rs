#![allow(dead_code, unused)]

use std::{fs::read_to_string, num};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

/// 1. Create 2D vector
/// 2. Check all around function
///     - Consider out of bounds
///     if not next to sumbol
///         continue
///     if next to symbol
///         while next to number
///             (num * 10) + next_num
///         add to sum
pub fn day3_p1() {
    let input = read_lines("src/day3/input.txt");

    for line in input {
        println!("{}", line);
    }
}

pub fn day3() {
    day3_p1();
}
