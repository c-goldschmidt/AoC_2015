use crate::days::day::{New, Parts};
use regex::Regex;

pub struct Day5 {
    input: Vec<String>,
}

impl Day5 {

    fn count_vowels(&self, line: &String) -> i32 {
        let re = Regex::new(r"[aeiou]").unwrap();
        return re.find_iter(line).count() as i32;
    }

    fn has_doubles(&self, line: &String) -> bool {
        let mut prev= "NULL";
        for char in line.split("") {
            if char == prev {
                return true;
            }
            prev = char;
        }

        return false;
    }

    fn is_naughty(&self, line: &String) -> bool {
        let re = Regex::new(r"(ab|cd|pq|xy)").unwrap();
        return re.is_match(line);
    }

    fn has_pairs(&self, line: &String) -> bool {
        let line_split: Vec<String> = line.split("").map(|x| String::from(x)).collect();

        for i in 1..line_split.len() - 2 {
            let pair = vec![
                line_split[i].clone(),
                line_split[i + 1].clone(),
            ].join("");

            let re = Regex::new(&pair).unwrap();
            if re.find_iter(line).count() > 1 {
                return true;
            }
        }

        return false;
    }

    fn has_repeat(&self, line: &String) -> bool {
        let line_split: Vec<String> = line.split("").map(|x| String::from(x)).collect();

        for i in 1..line_split.len() - 1 {
            let search = vec![
                line_split[i].clone(),
                ".".to_string(),
                line_split[i].clone(),
            ].join("");
            let re = Regex::new(&search).unwrap();
            if re.is_match(line) {
                return true;
            }
        }

        return false;
    }
}

impl New<Day5> for Day5 {
    fn new(file_name: &str) -> Day5 {
        Day5 { input: Day5::get_content(file_name)}
    }
}

impl Parts for Day5 {
    fn part1(&self) -> i32 {
        let mut nice = 0;
        for row in self.input.clone() {
            let vowels = self.count_vowels(&row);
            let has_doubles = self.has_doubles(&row);
            let is_naughty = self.is_naughty(&row);

            nice += if vowels > 2 && has_doubles && !is_naughty {1} else {0};
        }

        return nice;
    }

    fn part2(&self) -> i32 {
        let mut nice = 0;
        for row in self.input.clone() {
            let has_pairs = self.has_pairs(&row);
            let has_repeat = self.has_repeat(&row);

            nice += if has_pairs && has_repeat {1} else {0};
        }
        return nice;
    }
}