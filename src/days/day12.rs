use crate::days::day::{New, Parts};
use regex::Regex;

pub struct Day12 {
    input: Vec<String>,
}

impl New<Day12> for Day12 {
    fn new(file_name: &str) -> Day12 {
        Day12 { input: Day12::get_content(file_name)}
    }
}

impl Day12 {
    fn sum_all_numbers(&self, input: &String) -> i32 {
        let re = Regex::new(r"[:,\[](?P<num>-?\d+)").unwrap();
        let mut total = 0;
        for cap in re.captures_iter(input) {
            total += cap.name("num").unwrap().as_str().parse::<i32>().unwrap();
        }

        return total;
    }

    fn contains_red(&self, json: &json::JsonValue) -> bool {
        for (key, value) in json.entries() {
            if key == "red" || value == "red" {
                return true;
            }
        }
        return false;
    }

    fn erase_red(&self, json: &json::JsonValue) -> json::JsonValue {
        let mut result = json.clone();
        for (key, value) in json.entries() {
            if self.contains_red(&value) {
                result.remove(key);
            } else {
                result[key] = self.erase_red(&value);
            }
        }

        let mut i = 0;
        for member in json.members() {
            if self.contains_red(&member) {
                result[i] = 0.into();
            } else {
                result[i] = self.erase_red(member);
            }

            i += 1;
        }
        return result;
    }

    fn sum_all_numbers_no_red(&self, input: &String) -> i32 {
        let mut parsed = json::parse(input).unwrap();
        parsed = self.erase_red(&mut parsed);
        println!("result: {}", &parsed.dump());
        return self.sum_all_numbers(&parsed.dump());
    }
}

impl Parts for Day12 {
    fn part1(&self) -> i32 {
        return self.sum_all_numbers(&self.input[0]);
    }

    fn part2(&self) -> i32 {
        return self.sum_all_numbers_no_red(&self.input[0]);
    }
}