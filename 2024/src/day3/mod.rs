use std::fs;

use regex::Regex;

use crate::utils;

pub fn run_part1() {
    let mut total = 0;
    let match_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    if let Ok(lines) = utils::read_lines("./src/day3/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                let matches = match_re.captures_iter(&line_data);

                for m in matches {
                    let Some(first) = m.get(1) else {
                        panic!("WTF");
                    };
                    let Some(second) = m.get(2) else {
                        panic!("WTF");
                    };

                    total +=
                        first.as_str().parse::<i32>().unwrap() *
                        second.as_str().parse::<i32>().unwrap();
                }
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    println!("{}", total);
}

pub fn run_part2() {
    let mut total = 0;

    if let Ok(content) = fs::read_to_string("./src/day3/input.txt") {
        let do_split = content.split("do()");
        println!("there are {} splits of dos", do_split.clone().count());

        for split in do_split {
            if let Some(next_dont) = split.find("don't()") {
                total += calculate_mult(&split[..next_dont]);
            } else {
                total += calculate_mult(&split);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    println!("{}", total);
}

fn calculate_mult(split: &str) -> i32 {
    let match_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let matches = match_re.captures_iter(&split);
    let mut total = 0;

    for m in matches {
        let Some(first) = m.get(1) else {
            panic!("WTF");
        };
        let Some(second) = m.get(2) else {
            panic!("WTF");
        };

        total += first.as_str().parse::<i32>().unwrap() * second.as_str().parse::<i32>().unwrap();
    }

    total
}
