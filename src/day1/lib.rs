#![allow(dead_code, unused)]

use std::{fs::read_to_string, num};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn day1_p1() -> i32 {
    let input = read_lines("src/day1/input.txt");
    let mut res = Vec::new();

    for s in input {
        let first_num: &char;
        let last_num: &char;

        let all_nums: Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();

        first_num = all_nums.first().unwrap();
        last_num = all_nums.last().unwrap();

        let num = first_num.to_string() + &last_num.to_string();
        let str_num: i32 = num.parse().unwrap();

        res.push(str_num);
    }

    res.iter().sum::<i32>()
}

///idk what the fuck im doing i gave up...
pub fn day1_p2() -> i32 {
    let input = read_lines("src/day1/input.txt");
    let digit_in_str = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digit_as_str = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut res = Vec::new();

    for s in input {
        let first_num: &char;
        let last_num: &char;

        println!("{}", s);
        // let mut filtered_string = String::from(s);
        //
        // println!("{}", filtered_string);
        // for (i, j) in digit_in_str.into_iter().zip(digit_as_str.into_iter()) {
        //     filtered_string = filtered_string.replace(i, j);
        // }
        // println!("{}", filtered_string);

        let all_nums: Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();
        let mut integer_index: Vec<usize> = s
            .char_indices()
            .filter_map(|(index, c)| if c.is_numeric() { Some(index) } else { None })
            .collect();

        let mut digit_as_str_index: Vec<usize> = Vec::new();
        let mut digit_as_str_value: Vec<&str> = Vec::new();

        for d in digit_in_str {
            if s.find(d).is_some() {
                digit_as_str_index.push(s.find(d).unwrap());
                digit_as_str_value.push(d);
            }
        }
        println!("digit as str index{:?}", digit_as_str_index);
        println!("digit as str value{:?}", digit_as_str_value);
        println!("int index{:?}", integer_index);

        let mut combined_index = Vec::new();
        combined_index.append(&mut integer_index);
        combined_index.append(&mut digit_as_str_index);

        println!("comb {:?}", combined_index);

        let min = combined_index.iter().min().unwrap();
        println!("min {}", min);
        // first_num = if integer_index.contains(min) {
        //     &s.chars().nth(min.clone()).unwrap()
        // } else {

        let idx = match digit_as_str_index
            .iter()
            .position(|&r| -> bool { r == *min })
        {
            Some(val) => val,
            None => 100,
        };
        println!("idx {}", idx);
        let num_as_string = digit_as_str_value.iter().nth(idx).unwrap();
        println!("num_as_string{}", num_as_string);
        let temp = digit_in_str
            .iter()
            .position(|x| x == num_as_string)
            .unwrap();
        let temp = digit_as_str.iter().nth(temp).unwrap();
        println!("res{}", temp);

        first_num = all_nums.last().unwrap();
        last_num = all_nums.last().unwrap();

        let num = first_num.to_string() + &last_num.to_string();
        let str_num: i32 = num.parse().unwrap();

        println!("{}", str_num);

        res.push(str_num);
    }

    res.iter().sum::<i32>()
}

pub fn day1() {
    // println!("{}", day1_p1());
    // println!("{}", day1_p2());
}
