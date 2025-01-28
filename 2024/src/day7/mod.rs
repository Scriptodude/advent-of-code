use crate::utils;
use itertools::Itertools;

pub fn run_part1() {
    let mut data = Day7::new();

    if let Ok(lines) = utils::read_lines("./src/day7/sample.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                data.push_line(&line_data);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    println!("day7 part1: {}", data.do_part1());
}

pub fn run_part2() {
    let mut data = Day7::new();

    if let Ok(lines) = utils::read_lines("./src/day7/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                data.push_line(&line_data);
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    println!("day7 part2: {}", data.do_part2());
}

struct Equation {
    total: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn new(line: &str) -> Self {
        let split = line.split_once(":").unwrap();
        let total = split.0.parse::<usize>().unwrap();
        let numbers = split.1
            .split(" ")
            .map(|v| v.parse::<usize>())
            .filter(|v| v.is_ok())
            .map(|v| v.unwrap());

        Self { total, numbers: Vec::from_iter(numbers) }
    }

    fn is_solvable(&self) -> bool {
        let total = &Vec::from_iter(["+", "*"].repeat(self.numbers.len()));
        let permutations = total.iter().combinations(self.numbers.len() - 1);

        for perm in permutations {
            if self.eval_equation(&perm) == self.total {
                return true;
            }
        }

        return false;
    }

    fn is_solvable2(&self) -> bool {
        let total = &Vec::from_iter(["+", "*", "||"].repeat(self.numbers.len()));
        let permutations = total.iter().combinations(self.numbers.len() - 1);

        for perm in permutations {
            let evalued = self.eval_equation(&perm);
            if evalued == self.total {
                return true;
            }
        }

        return false;
    }

    fn eval_equation(&self, perm: &Vec<&&str>) -> usize {
        let mut acc = self.numbers[0];

        let mut apply_op = |n: usize, op: &str| {
            match op {
                "+" => acc += n,
                "*" => acc *= n,
                "||" => acc = format!("{}{}", acc, n).parse().unwrap(),
                _ => panic!("WTF")
            }
        };

        for i in 1..self.numbers.len() {
            apply_op(self.numbers[i], perm[i-1]);
        }
    
        return acc;
    }
}

struct Day7 {
    equations: Vec<Equation>,
}

impl Day7 {
    fn new() -> Self {
        Self { equations: Vec::new() }
    }

    fn push_line(&mut self, line: &str) {
        self.equations.push(Equation::new(line));
    }

    fn do_part1(&self) -> usize {
        let eq_size = self.equations.len();
        let mut total = 0;
        for (i, eq) in self.equations.iter().enumerate() {
            println!("treating {} of {}", i, eq_size);

            if eq.is_solvable() {
                total += eq.total;
            }
        }

        return total;
    }

    fn do_part2(&self) -> usize {
        let eq_size = self.equations.len();
        let mut total = 0;
        for (i, eq) in self.equations.iter().enumerate() {
            println!("treating {} of {}", i, eq_size);

            if eq.is_solvable2() {
                total += eq.total;
            }
        }

        return total;
    }
}
