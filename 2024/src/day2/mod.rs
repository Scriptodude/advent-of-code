use crate::utils;

pub fn run_part1() {
    let mut day2 = Day2::new();

    if let Ok(lines) = utils::read_lines("./src/day2/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                day2.push_line(&line_data);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    println!("{}", day2.count_safe(false));
}


pub fn run_part2() {
    let mut day2 = Day2::new();

    if let Ok(lines) = utils::read_lines("./src/day2/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                day2.push_line(&line_data);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    println!("{}", day2.count_safe(true));
}

struct Day2 {
    reports: Vec<Vec<i32>>
}

impl Day2 {
    fn new() -> Self {
        Self { reports: Vec::new() }
    }

    fn push_line(&mut self, line: &String) {
        let data = line.trim().split(" ").map(|d| d.parse::<i32>().unwrap());

        let mut data_vec: Vec<i32> = Vec::new();
        for i in data {
            data_vec.push(i);
        }

        self.reports.push(data_vec);
    }

    fn count_safe(&self, use_part2: bool) -> i32 {
        let mut total = 0;

        for report in self.reports.as_slice() {
            if Self::is_report_safe(report) {
                total += 1;
            } else if use_part2 {

                for i in 0..report.len() {
                    let mut clone = report.clone();
                    clone.remove(i);
                    if Self::is_report_safe(&clone) {
                        total += 1;
                        break;
                    }
                }

            }
        }

        total
    }

    fn is_report_safe(report: &[i32]) -> bool {
        let ascending = report.is_sorted();
        for i in 1..report.len() {
            if ascending && report[i] <= report[i-1] || !ascending && report[i] >= report[i-1] {
                return false;
            }

            let diff = (report[i] - report[i-1]).abs();
            if diff > 3 || diff < 1  {
                return false;
            }
        }

        true
    }
}