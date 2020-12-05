use crate::days::day::{New, Parts};
use regex::{Regex, Captures};

pub struct Day6 {
    input: Vec<String>,
}

pub struct Grid {
    lights: Vec<Vec<bool>>,
}

pub struct LightGrid {
    lights: Vec<Vec<i32>>,
}

trait Apply {
    fn set(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, on: bool);
    fn toggle(&mut self, x0: usize, y0: usize, x1: usize, y1: usize);

    fn apply_match(&mut self, cap: Captures) {
        let x0 = cap.name("x0").unwrap().as_str().parse::<usize>().unwrap();
        let y0 = cap.name("y0").unwrap().as_str().parse::<usize>().unwrap();
        let x1 = cap.name("x1").unwrap().as_str().parse::<usize>().unwrap();
        let y1 = cap.name("y1").unwrap().as_str().parse::<usize>().unwrap();
        let action = cap.name("action").unwrap().as_str();

        match action {
            "toggle" => self.toggle(x0, y0, x1, y1),
            "turn on" => self.set(x0, y0, x1, y1, true),
            "turn off" => self.set(x0, y0, x1, y1, false),
            _ => {
                panic!("no idea what to do: {}", action);
            }
        }
    }

    fn apply_line(&mut self, line: &String) {
        let rx = r"(?P<action>\w+(?: \w+)?) (?P<x0>\d+),(?P<y0>\d+) through (?P<x1>\d+),(?P<y1>\d+)";
        let re = Regex::new(rx).unwrap();
        let matched = re.captures(line);

        match matched {
            None => panic!("No idea what to do: {}", line),
            Some(capture) => {
                self.apply_match(capture);
            }
        }
    }
}


impl Grid {
    pub fn new() -> Self {
        let mut lights = Vec::with_capacity(1000);
        for _i in 0..1000 {
            let mut sublights = Vec::with_capacity(1000);
            for _j in 0..1000 {
                sublights.push(false);
            }
            lights.push(sublights);
        }

        Grid {lights}
    }

    pub fn count_enabled(&self) -> i32 {
        let mut count = 0;
        for row in &self.lights {
            for col in row {
                count += if *col {1} else {0};
            }
        }
        return count;
    }
}

impl Apply for Grid {
    fn set(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, to: bool) {
        for x in x0..x1 + 1 {
            for y in y0..y1 + 1 {
                self.lights[x][y] = to;
            }
        }
    }

    fn toggle(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for x in x0..x1 + 1 {
            for y in y0..y1 + 1 {
                self.lights[x][y] = !self.lights[x][y];
            }
        }
    }
}

impl LightGrid {
    pub fn new() -> Self {
        let mut lights = Vec::with_capacity(1000);
        for _i in 0..1000 {
            let mut sublights = Vec::with_capacity(1000);
            for _j in 0..1000 {
                sublights.push(0);
            }
            lights.push(sublights);
        }

        LightGrid {lights}
    }

    pub fn total_luminance(&self) -> i32 {
        let mut total = 0;
        for row in &self.lights {
            for col in row {
                total += *col;
            }
        }
        return total;
    }
}

impl Apply for LightGrid {
    fn set(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, to: bool) {
        for x in x0..x1 + 1 {
            for y in y0..y1 + 1 {
                if !to && self.lights[x][y] == 0 {
                    continue;
                }
                self.lights[x][y] += if to {1} else {-1};
            }
        }
    }

    fn toggle(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for x in x0..x1 + 1 {
            for y in y0..y1 + 1 {
                self.lights[x][y] += 2;
            }
        }
    }
}

impl New<Day6> for Day6 {
    fn new(file_name: &str) -> Day6 {
        Day6 { input: Day6::get_content(file_name)}
    }
}

impl Parts for Day6 {
    fn part1(&self) -> i32 {
        let mut grid = Grid::new();
        for line in self.input.clone() {
            grid.apply_line(&line);
        }
        return grid.count_enabled();
    }

    fn part2(&self) -> i32 {
        let mut grid = LightGrid::new();
        for line in self.input.clone() {
            grid.apply_line(&line);
        }
        return grid.total_luminance();
    }
}