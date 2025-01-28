use std::{
    collections::{ HashMap, HashSet },
    fs,
    hash::{ Hash, Hasher },
    usize,
};

pub fn run_part1() {
    if let Ok(content) = fs::read_to_string("./src/day6/input.txt") {
        let mut day6 = Day6::new(&content);
        println!("day6 - part1: {}", day6.do_part1().len());
    } else {
        panic!("File not found input.txt");
    }
}

pub fn run_part2() {
    if let Ok(content) = fs::read_to_string("./src/day6/input.txt") {
        let day6 = Day6::new(&content);
        println!("day6 - part2: {}", day6.do_part2());
    } else {
        panic!("File not found input.txt");
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        return Position { x, y };
    }

    fn all_positions_to(&self, other: &Position) -> Vec<Position> {
        let mut positions = Vec::new();

        if self.x == other.x {
            if self.y > other.y {
                for y in other.y + 1..self.y {
                    positions.push(Position::new(self.x, y as usize));
                }
            } else {
                for y in self.y..other.y {
                    positions.push(Position::new(self.x, y as usize));
                }
            }
        } else {
            if self.x > other.x {
                for x in other.x + 1..self.x {
                    positions.push(Position::new(x as usize, self.y));
                }
            } else {
                for x in self.x..other.x {
                    positions.push(Position::new(x as usize, self.y));
                }
            }
        }

        return positions;
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

struct Day6 {
    obstacles: Vec<Position>,
    guard: Position,
    guard_direction: Direction,
    map_width: usize,
    guard_seen_states: HashMap<Direction, Vec<Position>>,
}

impl Clone for Day6 {
    fn clone(&self) -> Self {
        let mut map: HashMap<Direction, Vec<Position>> = HashMap::new();
        map.insert(Direction::Up, Vec::new());
        map.insert(Direction::Down, Vec::new());
        map.insert(Direction::Left, Vec::new());
        map.insert(Direction::Right, Vec::new());

        Self {
            obstacles: self.obstacles.clone(),
            guard: self.guard.clone(),
            guard_direction: self.guard_direction.clone(),
            map_width: self.map_width.clone(),
            guard_seen_states: map,
        }
    }
}

impl Day6 {
    fn new(map: &str) -> Self {
        let map_width = map.find("\n").unwrap() + 1;
        let guard_idx = map.find("^").unwrap();

        println!("{}", map_width);
        let mut obstacles = Vec::new();

        for (i, c) in map.char_indices() {
            if c == '#' {
                obstacles.push(Position::new(i % map_width, i / map_width));
            }
        }

        let mut map: HashMap<Direction, Vec<Position>> = HashMap::new();
        map.insert(Direction::Up, Vec::new());
        map.insert(Direction::Down, Vec::new());
        map.insert(Direction::Left, Vec::new());
        map.insert(Direction::Right, Vec::new());

        Self {
            obstacles,
            guard: Position::new(guard_idx % map_width, guard_idx / map_width),
            guard_direction: Direction::Up,
            map_width,
            guard_seen_states: map,
        }
    }

    fn do_part1(&mut self) -> HashSet<Position> {
        let mut distinct_positions: HashSet<Position> = HashSet::new();

        println!("obstacles: {:?}", self.obstacles);

        'l: loop {
            match self.guard_direction {
                Direction::Up => {
                    if
                        let Some(n) = self.obstacles
                            .iter()
                            .filter(|p| p.x == self.guard.x && p.y < self.guard.y)
                            .max_by_key(|p| p.y)
                    {
                        self.guard
                            .all_positions_to(n)
                            .iter()
                            .for_each(|v| {
                                distinct_positions.insert(v.clone());
                            });

                        self.guard.y = n.y + 1;
                        self.guard_direction = Direction::Right;
                    } else {
                        self.guard
                            .all_positions_to(&Position::new(self.guard.x, 0))
                            .iter()
                            .for_each(|v| {
                                distinct_positions.insert(v.clone());
                            });
                        distinct_positions.insert(Position::new(self.guard.x, 0));
                        break 'l;
                    }
                }
                Direction::Right => {
                    if
                        let Some(n) = self.obstacles
                            .iter()
                            .filter(|p| p.y == self.guard.y && p.x > self.guard.x)
                            .min_by_key(|p| p.x)
                    {
                        self.guard
                            .all_positions_to(n)
                            .iter()
                            .for_each(|v| {
                                distinct_positions.insert(v.clone());
                            });
                        self.guard.x = n.x - 1;
                        self.guard_direction = Direction::Down;
                    } else {
                        self.guard
                            .all_positions_to(&Position::new(self.map_width - 1, self.guard.y))
                            .iter()
                            .for_each(|v| {
                                distinct_positions.insert(v.clone());
                            });
                        distinct_positions.insert(Position::new(self.map_width - 1, self.guard.y));
                        break 'l;
                    }
                }
                Direction::Left => {
                    if
                        let Some(n) = self.obstacles
                            .iter()
                            .filter(|p| p.y == self.guard.y && p.x < self.guard.x)
                            .max_by_key(|p| p.x)
                    {
                        self.guard
                            .all_positions_to(n)
                            .iter()
                            .for_each(|v| {
                                distinct_positions.insert(v.clone());
                            });
                        self.guard.x = n.x + 1;
                        self.guard_direction = Direction::Up;
                    } else {
                        self.guard
                            .all_positions_to(&Position::new(0, self.guard.y))
                            .iter()
                            .for_each(|v| {
                                distinct_positions.insert(v.clone());
                            });
                        distinct_positions.insert(Position::new(0, self.guard.y));
                        break 'l;
                    }
                }
                Direction::Down => {
                    if
                        let Some(n) = self.obstacles
                            .iter()
                            .filter(|p| p.x == self.guard.x && p.y > self.guard.y)
                            .min_by_key(|p| p.y)
                    {
                        self.guard
                            .all_positions_to(n)
                            .iter()
                            .for_each(|v| {
                                distinct_positions.insert(v.clone());
                            });
                        self.guard.y = n.y - 1;
                        self.guard_direction = Direction::Left;
                    } else {
                        self.guard
                            .all_positions_to(&Position::new(self.guard.x, self.map_width - 1))
                            .iter()
                            .for_each(|v| {
                                distinct_positions.insert(v.clone());
                            });
                        distinct_positions.insert(Position::new(self.guard.x, self.map_width - 1));
                        break 'l;
                    }
                }
            }
        }

        return distinct_positions;
    }

    pub fn do_part2(&self) -> usize {
        let mut seen: HashSet<Position> = HashSet::new();

        for x in 0..self.map_width {
            for y in 0..self.map_width {
                let mut initial_map = self.clone();
                initial_map.obstacles.push(Position::new(x, y));

                'l: loop {
                    if initial_map.was_seen(&initial_map.guard) {
                        seen.insert(Position::new(x, y));
                        break 'l;
                    }

                    initial_map.guard_seen_states
                        .get_mut(&initial_map.guard_direction)
                        .unwrap()
                        .push(initial_map.guard.clone());

                    match initial_map.guard_direction {
                        Direction::Up => {
                            if
                                let Some(n) = initial_map.obstacles
                                    .iter()
                                    .filter(
                                        |p| p.x == initial_map.guard.x && p.y < initial_map.guard.y
                                    )
                                    .max_by_key(|p| p.y)
                            {
                                initial_map.guard.y = n.y + 1;
                                initial_map.guard_direction = Direction::Right;
                            } else {
                                break 'l;
                            }
                        }
                        Direction::Right => {
                            if
                                let Some(n) = initial_map.obstacles
                                    .iter()
                                    .filter(
                                        |p| p.y == initial_map.guard.y && p.x > initial_map.guard.x
                                    )
                                    .min_by_key(|p| p.x)
                            {
                                initial_map.guard.x = n.x - 1;
                                initial_map.guard_direction = Direction::Down;
                            } else {
                                break 'l;
                            }
                        }
                        Direction::Left => {
                            if
                                let Some(n) = initial_map.obstacles
                                    .iter()
                                    .filter(
                                        |p| p.y == initial_map.guard.y && p.x < initial_map.guard.x
                                    )
                                    .max_by_key(|p| p.x)
                            {
                                initial_map.guard.x = n.x + 1;
                                initial_map.guard_direction = Direction::Up;
                            } else {
                                break 'l;
                            }
                        }
                        Direction::Down => {
                            if
                                let Some(n) = initial_map.obstacles
                                    .iter()
                                    .filter(
                                        |p| p.x == initial_map.guard.x && p.y > initial_map.guard.y
                                    )
                                    .min_by_key(|p| p.y)
                            {
                                initial_map.guard.y = n.y - 1;
                                initial_map.guard_direction = Direction::Left;
                            } else {
                                break 'l;
                            }
                        }
                    }
                }
            }
        }

        return seen.len();
    }

    fn was_seen(&self, p: &Position) -> bool {
        return match self.guard_direction {
            Direction::Up => self.guard_seen_states[&Direction::Up].contains(p),
            Direction::Right => self.guard_seen_states[&Direction::Right].contains(p),
            Direction::Left => self.guard_seen_states[&Direction::Left].contains(p),
            Direction::Down => self.guard_seen_states[&Direction::Down].contains(p),
        };
    }
}
