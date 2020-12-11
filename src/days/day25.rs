use crate::days::day::{New, Parts};
use regex::Regex;

pub struct Day25 {
    input: Vec<String>,
}

pub struct Generator {
    search: (usize, usize),
}

impl Generator {
    pub fn new(input: &String) -> Self {
        // row 3010, column 3019
        let rx: Regex = Regex::new(r"row (?P<x>\d+), column (?P<y>\d+)").unwrap();
        let cap = rx.captures(input).unwrap();
        Generator {
            search: (
                cap.name("x").unwrap().as_str().parse::<usize>().unwrap() - 1,
                cap.name("y").unwrap().as_str().parse::<usize>().unwrap() - 1,
            ),
        }
    }

    pub fn next(row: usize, col: usize) -> (usize, usize) {
        if row == 0 {
            return (row + col + 1, 0);
        }
        return (row - 1, col + 1)
    }

    pub fn generate(&self) -> i32 {
        let mut index: (usize, usize) = (0, 0);
        let mut code: u64 = 20151125;
        let multiplier = 252533;
        let divider = 33554393;

        while index != self.search {
            index = Self::next(index.0, index.1);
            code = (code * multiplier) % divider;
        }

        println!("actual result: {}", code);
        return code as i32;
    }
}

impl New<Day25> for Day25 {
    fn new(file_name: &str) -> Day25 {
        Day25 { input: Day25::get_content(file_name)}
    }
}

impl Parts for Day25 {
    fn part1(&self) -> i32 {
        let generator = Generator::new(&self.input[0]);
        return generator.generate();
    }

    fn part2(&self) -> i32 {
        return 0;
    }
}