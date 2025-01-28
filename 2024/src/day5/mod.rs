use std::usize;

use crate::utils;

pub fn run_part1() {
    let mut data = Day5::new();

    if let Ok(lines) = utils::read_lines("./src/day5/sample.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                data.push_line(&line_data);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    data.do_part1();
}

pub fn run_part2() {
    let mut data = Day5::new();

    if let Ok(lines) = utils::read_lines("./src/day5/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                data.push_line(&line_data);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    data.do_part2();
}

struct Day5 {
    rules: Vec<Vec<usize>>,
    correct_pages: Vec<Vec<usize>>,
    incorrect_pages: Vec<Vec<usize>>,
}

impl Day5 {
    fn new() -> Self {
        let mut rules: Vec<Vec<usize>> = Vec::new();
        for _ in 0..100 {
            rules.push(Vec::new());
        }

        Self { rules: rules, correct_pages: Vec::new(), incorrect_pages: Vec::new() }
    }

    fn push_line(&mut self, line: &String) {
        // rules
        if let Some((left, right)) = line.split_once("|") {
            self.rules[left.parse::<usize>().unwrap()].push(right.parse().unwrap())
        } else if line.trim().is_empty() {
            return;
        } else {
            let split = Vec::from_iter(line.split(",").map(|v| v.parse::<usize>().unwrap()));

            for i in 1..split.len() {
                let rules = &self.rules[split[i]];

                if rules.iter().any(|x| split[0..i].contains(x)) {
                    self.incorrect_pages.push(split);
                    return;
                }
            }

            self.correct_pages.push(split);
        }
    }

    fn do_part1(&self) {
        let mut total = 0;
        for page in &self.correct_pages {
            total += page[page.len() / 2];
        }

        println!("day5 part1: {}", total);
    }

    fn do_part2(&self) {
        let mut corrected: Vec<Vec<usize>> = Vec::new();

        for page in &self.incorrect_pages {
            let mut copy = page.clone();

            while !self.is_correct_update(&copy) {
                for i in 1..copy.len() {
                    let rules = &self.rules[copy[i]];

                    if let Some(swap) = copy[0..i].iter().position(|x| rules.contains(x)) {
                        copy.swap(i, swap);
                    }
                }
            }

            corrected.push(copy);
        }

        let mut total = 0;
        for page in &corrected {
            total += page[page.len() / 2];
        }

        println!("day5 part2: {}", total);
    }

    fn is_correct_update(&self, pages: &Vec<usize>) -> bool {
        for i in 1..pages.len() {
            let rules = &self.rules[pages[i]];

            if rules.iter().any(|x| pages[0..i].contains(x)) {
                return false;
            }
        }

        return true;
    }
}
