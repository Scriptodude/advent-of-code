use std::{ fs, sync::{mpsc}, thread };

pub fn run_part1() {
    if let Ok(content) = fs::read_to_string("./src/day11/input.txt") {
        let data = Day11::new(&content);

        data.apply_rules(25);
    } else {
        panic!("File not found input.txt");
    }
}

pub fn run_part2() {
    if let Ok(content) = fs::read_to_string("./src/day11/input.txt") {
        let data = Day11::new(&content);

        data.apply_rules(75);
    } else {
        panic!("File not found test.txt");
    }
}

struct Day11 {
    iteration: usize,
    stones: Vec<usize>,
}

impl Day11 {
    fn new(data: &str) -> Self {
        let stones = &Vec::from_iter(data.split(" ").map(|n| n.parse::<usize>().unwrap()));

        Self { iteration: 0, stones: stones.clone() }
    }

    fn apply_rules(&self, total_iterations: usize) {
        let mut total = 0;
        let (tx, rx) = mpsc::sync_channel(self.stones.len());

        let mut threads = Vec::new();

        for stone in &self.stones {
            let tx = tx.clone();
            let stone = stone.clone();

            threads.push(thread::spawn(move || {
                let (i, nb) = Self::bring_to_even_len(stone);
                let result = Self::do_stuff(nb, i, total_iterations);

                println!("sending {}", result);
                tx.send(result).unwrap();
            }));
        }

        for thread in threads {
            total += rx.recv().unwrap();
            thread.join().unwrap();
        }

        println!("{:?}", total);
    }

    fn bring_to_even_len(num: usize) -> (usize, usize) {
        if num == 0 {
            return (2, 2024);
        }
        let mut i = 0;
        let mut n = num;

        loop {
            if n.to_string().len() % 2 == 0 {
                break;
            }

            n *= 2024;
            i += 1;
        }

        return (i, n);
    }

    fn do_stuff(num: usize, current_iter: usize, max_iter: usize) -> usize {
        if current_iter >= max_iter {
            return 1;
        }

        if num < 10 || num.to_string().len() % 2 != 0 {
            let (i, n) = Self::bring_to_even_len(num);

            if current_iter + i >= max_iter {
                return 1;
            }

            return Self::do_stuff(n, current_iter + i, max_iter);
        }        

        let left = num.to_string()[..num.to_string().len()/2].parse::<usize>().unwrap();
        let right = num.to_string()[num.to_string().len()/2..].parse::<usize>().unwrap();

        let total = Self::do_stuff(left, current_iter + 1, max_iter);
        return Self::do_stuff(right, current_iter + 1, max_iter) + total;
    }
}
