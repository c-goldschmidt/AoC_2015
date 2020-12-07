use crate::days::day::{New, Parts};
use std::collections::HashSet;

pub struct Day11 {
    input: Vec<String>,
}

impl New<Day11> for Day11 {
    fn new(file_name: &str) -> Day11 {
        Day11 { input: Day11::get_content(file_name)}
    }
}

impl Day11 {
    fn increase_string(&self, input: &String) -> String {
        let split: Vec<&str> = input.split("").collect();
        let mut increased = String::from("");
        let mut inc_next = true;
        for i in 1..split.len() - 1 {
            let index = split.len() - 1 - i;
            let mut current = split[index].as_bytes()[0];

            if inc_next && current + 1 as u8 > 122 as u8 {
                current = 97;
            } else if inc_next {
                inc_next = false;
                current += 1;
            }
            increased = String::from(current as char) + &increased;
        }

        return increased;
    }

    fn check_valid(&self, input: &String) -> bool {
        let split: Vec<&str> = input.split("").collect();
        let mut prev: Option<&str> = None;
        let mut doubles = HashSet::new();
        let mut straight_active = true;
        let mut current_straight = 0;
        let mut max_straight = 0;

        for i in 1..split.len() - 1 {
            let current = split[i];

            if current == "i" || current == "o" || current == "l" {
                return false;
            }

            match prev {
                Some(value) => {
                    if current == value {
                        doubles.insert(format!("{}{}", &current, &current));
                    }

                    if current.as_bytes()[0] == value.as_bytes()[0] + 1 {
                        if straight_active {
                            current_straight += 1;
                        } else {
                            current_straight = 2;
                            straight_active = true;
                        }

                        if current_straight > max_straight {
                            max_straight = current_straight;
                        }
                    } else {
                        straight_active = false;
                    }

                },
                None => {},
            }
            prev = Some(current);
        }

        return doubles.len() > 1 && max_straight >= 3;
    }

    fn get_next(&self, input: &String) -> String {
        let mut current = input.clone();
        current = self.increase_string(&current);
        while !self.check_valid(&current) {
            current = self.increase_string(&current);
        }
        return current;
    }
}

impl Parts for Day11 {
    fn part1(&self) -> i32 {
        println!("increased: {}", &self.get_next(&self.input[0]));
        return 0; // non-numeric challenge...
    }

    fn part2(&self) -> i32 {
        println!("increased: {}", &self.get_next(&self.get_next(&self.input[0])));
        return 0;
    }
}