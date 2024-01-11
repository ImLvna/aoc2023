use crate::clap::{Args, InputType};

static EXAMPLE_INPUT: &str = include_str!("../../../inputs/1/example.txt");
static REAL_INPUT: &str = include_str!("../../../inputs/1/input.txt");

fn get_input(args: &Args) -> &'static str {
    match args.input {
        InputType::Example => EXAMPLE_INPUT,
        InputType::Final => REAL_INPUT,
    }
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn replace_left(line: &str) -> String {
    let mut new_line = String::from("");
    let chars: Vec<&str> = line.split("").collect();
    for _char in chars.iter() {
        new_line = new_line + _char;
        for (i, el) in NUMS.iter().enumerate() {
            new_line = new_line.replace(el, &(i + 1).to_string())
        }
    }
    return new_line.to_owned();
}
fn replace_right(line: &str) -> String {
    let mut new_line = String::from("");
    let reversed = line.chars().rev().collect::<String>();
    let chars: Vec<&str> = reversed.split("").collect();
    for _char in chars.iter() {
        new_line = new_line + _char;
        for (i, el) in NUMS.iter().enumerate() {
            new_line = new_line.replace(&el.chars().rev().collect::<String>(), &(i + 1).to_string())
        }
    }
    return new_line.to_owned();
}
fn get_num(line: String) -> Option<i32> {
    let chars: Vec<&str> = line.split("").collect();
    let digits: Vec<i32> = chars.iter().filter_map(|c| c.parse::<i32>().ok()).collect();

    return digits.first().copied();
}

pub fn day1(args: &Args) -> String {
    let contents = get_input(&args);
    let mut number = 0;
    for line in contents.lines() {
        match get_num(replace_right(line)) {
            None => {}
            Some(num) => number += (get_num(replace_left(line)).unwrap_or(0) * 10) + num,
        }
    }
    return number.to_string();
}
