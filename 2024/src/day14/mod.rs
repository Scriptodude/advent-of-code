use crate::utils;

const MAP_SIZE_X: i32 = 101;
const MAP_SIZE_Y: i32 = 103;

//const MAP_SIZE_X: i32 = 11;
//const MAP_SIZE_Y: i32 = 7;

pub fn run_part1() {
    let mut robots_position = Vec::new();

    if let Ok(lines) = utils::read_lines("./src/day14/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                robots_position.push(Robot::new(&line_data).get_position_after(100));
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    let top_left = robots_position
        .iter()
        .filter(|(x, y)| *x < MAP_SIZE_X / 2 && *y < MAP_SIZE_Y / 2)
        .count();
    let top_right = robots_position
        .iter()
        .filter(|(x, y)| *x > MAP_SIZE_X / 2 && *y < MAP_SIZE_Y / 2)
        .count();
    let bottom_left = robots_position
        .iter()
        .filter(|(x, y)| *x < MAP_SIZE_X / 2 && *y > MAP_SIZE_Y / 2)
        .count();
    let bottom_right = robots_position
        .iter()
        .filter(|(x, y)| *x > MAP_SIZE_X / 2 && *y > MAP_SIZE_Y / 2)
        .count();

    println!("{}", top_left * top_right * bottom_left * bottom_right);
}

pub fn run_part2() {
    let mut robots = Vec::new();

    if let Ok(lines) = utils::read_lines("./src/day14/input.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                robots.push(Robot::new(&line_data));
            }
        }
    } else {
        panic!("File not found input.txt");
    }

    // 271 ? 328 ?

    for seconds in MAP_SIZE_X*3..MAP_SIZE_X*5 {
        let mut grid = [['_'; MAP_SIZE_X as usize]; MAP_SIZE_Y as usize];
        robots.iter().for_each(|r| {
            let (x, y) = r.get_position_after(seconds);
            grid[y as usize][x as usize] = 'X';
        });

        for y in 0..MAP_SIZE_Y {
            println!("{:?}", grid[y as usize]);
        }

        println!("{}", seconds);
    }
}

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn new(line: &str) -> Self {
        let (p, v) = line.split_once(" ").unwrap();

        let (px, py) = p[2..].split_once(",").unwrap();
        let (vx, vy) = v[2..].split_once(",").unwrap();

        Self {
            px: px.parse().unwrap(),
            py: py.parse().unwrap(),
            vx: vx.parse().unwrap(),
            vy: vy.parse().unwrap(),
        }
    }

    fn get_position_after(&self, seconds: i32) -> (i32, i32) {
        let mut new_px = (self.px + self.vx * seconds) % MAP_SIZE_X;
        let mut new_py = (self.py + self.vy * seconds) % MAP_SIZE_Y;

        if new_px < 0 {
            new_px = MAP_SIZE_X + new_px;
        }

        if new_py < 0 {
            new_py = MAP_SIZE_Y + new_py;
        }

        return (new_px, new_py);
    }
}
