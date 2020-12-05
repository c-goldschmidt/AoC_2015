use std::collections::HashSet;

use crate::days::day::{New, Parts};

pub struct Day3 {
    input: Vec<String>,
}

pub struct Tracker {
    x: i32,
    y: i32,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker {x: 0, y: 0}
    }

    pub fn move_to(&mut self, direction: &str, visited: &mut HashSet<Vec<i32>>) {
        match direction {
            "^" => self.x += 1,
            "v" => self.x -= 1,
            ">" => self.y += 1,
            "<" => self.y -= 1,
            _ => {},
        }

        let pos = vec![self.x, self.y];
        if !visited.contains(&pos) {
            visited.insert(pos);
        }
    }
}

impl New<Day3> for Day3 {
    fn new(file_name: &str) -> Day3 {
        Day3 { input: Day3::get_content(file_name)}
    }
}

impl Parts for Day3 {
    fn part1(&self) -> i32 {
        let mut visited = HashSet::new();
        let mut tracker = Tracker::new();

        for char in self.input[0].split("") {
            tracker.move_to(&char, &mut visited);
        }

        return visited.len() as i32;
    }

    fn part2(&self) -> i32 {
        let mut visited = HashSet::new();
        let mut tracker1 = Tracker::new();
        let mut tracker2 = Tracker::new();
        let mut i = 0;

        for char in self.input[0].split("") {
            if i.clone() % 2 == 0 {
                tracker1.move_to(&char, &mut visited);
            } else {
                tracker2.move_to(&char, &mut visited);
            }
            i += 1;
        }

        return visited.len() as i32;
    }
}