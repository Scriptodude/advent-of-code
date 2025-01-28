// use crate::utils;

use std::{cmp::min, fs, ops::Add};

pub fn run_part1() {
    if let Ok(content) = fs::read_to_string("./src/day4/input.txt") {
        let day4 = Day4::new(&content);
        println!("day4 - part1: {}", day4.count_xmas())
    } else {
        panic!("File not found input.txt");
    }
}

pub fn run_part2() {
    if let Ok(content) = fs::read_to_string("./src/day4/input.txt") {
        let day4 = Day4::new(&content);
        println!("day4 - part2: {}", day4.count_x_mas())
    } else {
        panic!("File not found input.txt");
    }
}

struct Day4<'a> {
    data: &'a str,
    line_length: usize
}

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

impl<'a> Day4<'a> {
    fn new(content: &'a String) -> Self {
        Self { data: content, line_length: content.find("\n").unwrap_or_default().add(1) }
    }

    fn count_xmas(&self) -> usize {
        let mut count = 0;
        let mut next_x: usize = 0;

        loop {
            // println!("--------------");
            // println!("current x: {}", next_x);
            

            // Horizontal Right
            // println!("horizontal_right: {}", self.data[next_x..min(next_x+4, self.data.len())].to_string());
            if self.data[next_x..min(next_x+4, self.data.len())] == *XMAS {
                count += 1;
            }

            // Horizontal Left
            // println!("horizontal_left: {}", self.data[next_x.checked_sub(3).unwrap_or(0)..next_x+1].to_string());
            if self.data[next_x.checked_sub(3).unwrap_or(0)..next_x+1] == *SAMX {
                count += 1;
            }

            // Vertical Top
            if let Some(_) = self.extract_vertical_bot(next_x) {
                count += 1;
            }

            if let Some(_) = self.extract_vertical_top(next_x) {
                count += 1;
            }

            if let Some(_) = self.extract_diagonal_bottom_left(next_x) {
                count += 1
            }

            if let Some(_) = self.extract_diagonal_bottom_right(next_x) {
                count += 1
            }

            if let Some(_) = self.extract_diagonal_top_left(next_x) {
                count += 1
            }

            if let Some(_) = self.extract_diagonal_top_right(next_x) {
                count += 1
            }

            // println!("count at : {}", count);

            match self.data[next_x+1..].find("X") {
                Some(v) => next_x += v + 1,
                None => break,
            };
        }

        count
    }

    fn count_x_mas(&self) -> usize {
        let mut count = 0;
        let mut idx: usize = 0;

        loop {
            // println!("--------------");
            // println!("current m: {}", idx);

            if idx + 2 * self.line_length + 2 < self.data.len() {
                let bottom_right = format!("{}{}{}", self.data.chars().nth(idx).unwrap(), self.data.chars().nth(idx + self.line_length + 1).unwrap(), self.data.chars().nth(idx + 2 * self.line_length + 2).unwrap());

                // println!("bot_right: {}", bottom_right);

                if bottom_right == "MAS" || bottom_right == "SAM" {        

                    if idx + 2 * self.line_length < self.data.len() {
                        let other = format!("{}{}{}", self.data.chars().nth(idx+2).unwrap(), self.data.chars().nth(idx + self.line_length + 1).unwrap(), self.data.chars().nth(idx + 2 * self.line_length).unwrap());

                        // println!("other: {}", bottom_right);

                        if other == "MAS" || other == "SAM" {
                            count += 1;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
    
            // println!("count at : {}", count);

            idx += 1;
        }

        count
    }

    fn extract_vertical_bot(&self, idx: usize) -> Option<bool> {
        if let None = idx.checked_add(3 * self.line_length) {
            return None
        }

        if idx + 3 * self.line_length >= self.data.len() {
            return None
        }

        let format = format!("{}{}{}{}", self.data.chars().nth(idx).unwrap(), self.data.chars().nth(idx + self.line_length).unwrap(), self.data.chars().nth(idx + 2 * self.line_length).unwrap(), self.data.chars().nth(idx + 3 * self.line_length).unwrap());
        // println!("vertical_bot: {}", format);

        return if format == XMAS { Some(true) } else { None };
    }

    fn extract_vertical_top(&self, idx: usize) -> Option<bool> {
        if let None = idx.checked_sub(3 * self.line_length) {
            return None
        }

        let format = format!("{}{}{}{}", self.data.chars().nth(idx).unwrap(), self.data.chars().nth(idx - self.line_length).unwrap(), self.data.chars().nth(idx - 2 * self.line_length).unwrap(), self.data.chars().nth(idx - 3 * self.line_length).unwrap());
        // println!("vertical_top: {}", format);

        return if format == XMAS { Some(true) } else { None };
    }

    fn extract_diagonal_top_left(&self, idx: usize) -> Option<bool> {
        if let None = idx.checked_sub(3 * self.line_length + 3) {
            return None
        }

        let format = format!("{}{}{}{}", self.data.chars().nth(idx).unwrap(), self.data.chars().nth(idx - self.line_length - 1).unwrap(), self.data.chars().nth(idx - 2 * self.line_length - 2).unwrap(), self.data.chars().nth(idx - 3 * self.line_length - 3).unwrap());
        // println!("diagonal_top_left: {}", format);

        return if format == XMAS { Some(true) } else { None };
    }

    fn extract_diagonal_top_right(&self, idx: usize) -> Option<bool> {
        if let None = idx.checked_sub(3 * self.line_length - 3) {
            return None
        }

        let format = format!("{}{}{}{}", self.data.chars().nth(idx).unwrap(), self.data.chars().nth(idx - self.line_length + 1).unwrap(), self.data.chars().nth(idx - 2 * self.line_length + 2).unwrap(), self.data.chars().nth(idx - 3 * self.line_length + 3).unwrap());
        // println!("diagonal_top_right: {}", format);

        return if format == XMAS { Some(true) } else { None };
    }

    fn extract_diagonal_bottom_left(&self, idx: usize) -> Option<bool> {
        if let None = idx.checked_add(3 * self.line_length - 3) {
            return None
        }

        if idx + 3 * self.line_length - 3 >= self.data.len() {
            return None
        }

        let format = format!("{}{}{}{}", self.data.chars().nth(idx).unwrap(), self.data.chars().nth(idx + self.line_length - 1).unwrap(), self.data.chars().nth(idx + 2 * self.line_length - 2).unwrap(), self.data.chars().nth(idx + 3 * self.line_length - 3).unwrap());
        // println!("diagonal_bot_left: {}", format);

        return if format == XMAS { Some(true) } else { None };
    }

    fn extract_diagonal_bottom_right(&self, idx: usize) -> Option<bool> {
        if let None = idx.checked_add(3 * self.line_length + 3) {
            return None
        }

        if idx + 3 * self.line_length + 3 >= self.data.len() {
            return None
        }

        let format = format!("{}{}{}{}", self.data.chars().nth(idx).unwrap(), self.data.chars().nth(idx + self.line_length + 1).unwrap(), self.data.chars().nth(idx + 2 * self.line_length + 2).unwrap(), self.data.chars().nth(idx + 3 * self.line_length + 3).unwrap());
        // println!("diagonal_bottom_right: {}", format);

        return if format == XMAS { Some(true) } else { None };
    }
}
