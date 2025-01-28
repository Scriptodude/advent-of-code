use crate::utils;

pub fn run_day1_part1() {
    let mut day1 = Day1::new();

    if let Ok(lines) = utils::read_lines("./src/day1/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                day1.push_line(&line_data);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    println!("{}", day1.calculate_diff());
}

pub fn run_day1_part2() {
    let mut day1 = Day1::new();

    if let Ok(lines) = utils::read_lines("./src/day1/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                day1.push_line(&line_data);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    println!("{}", day1.calculate_similarity());
}

struct Day1 {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Day1 {
    fn new() -> Self {
        Self { left: Vec::new(), right: Vec::new() }
    }

    fn push_line(&mut self, line: &String) {
        if let Some(data) = line.trim().split_once("   ") {
            self.left.push(data.0.parse().unwrap());
            self.left.sort();
            self.right.push(data.1.parse().unwrap());
            self.right.sort();
        }
    }

    fn calculate_diff(&self) -> i32 {
        let mut total = 0;
        for i in 0..self.left.len() {
            total += (self.left[i] - self.right[i]).abs();
        }

        total
    }

    fn calculate_similarity(&self) -> i32 {
        let mut total = 0;
        for value in self.left.clone() {
            for right in self.right.clone() {
                if right > value {
                    break;
                }

                if right == value {
                    total += value;
                }
            }
        }

        total
    }
}
