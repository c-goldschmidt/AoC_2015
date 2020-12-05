use crate::days::day::{New, Parts};

pub struct Day1 {
    input: Vec<String>,
}

impl New<Day1> for Day1 {
    fn new(file_name: &str) -> Day1 {
        Day1 { input: Day1::get_content(file_name)}
    }
}

impl Parts for Day1 {
    fn part1(&self) -> i32 {
        let count_open = self.input[0].matches("(").count() as i32;
        let count_close = self.input[0].matches(")").count() as i32;
        return count_open - count_close;
    }

    fn part2(&self) -> i32 {
        let mut curr = 0;
        let mut index = 0;

        for char in self.input[0].split("") {
            if char == "" {
                continue;
            }

            curr += if char == "(" {1} else {-1};

            index += 1;
            if curr < 0 {
                return index;
            }
        }
        panic!("Not found");
    }
}