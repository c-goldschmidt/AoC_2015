use crate::days::day::{New, Parts};
use std::collections::{HashMap};
use crate::days::utils::LineMatch;
use regex::{Regex, Captures};
use itertools::Itertools;

pub struct Day19 {
    input: Vec<String>,
}

struct MolMix {
    replacements: HashMap<String, Vec<String>>,
    pub input: String,
}

impl MolMix {
    pub fn new() -> Self {
        MolMix { replacements: HashMap::new(), input: String::new()}
    }

    fn calculate_result(&mut self, input: &String) {
        self.input = input.clone();
    }

    pub fn count_result(&self) -> i32 {
        return self.get_replacements(&self.input).into_iter().unique().count() as i32;
    }

    fn get_replacements(&self, source: &String) -> Vec<String> {
        let mut results = Vec::new();

        for (replace, with) in &self.replacements {
            for (item, replacement) in Regex::new(replace).unwrap().find_iter(source).cartesian_product(with) {
                let mut copy = source.clone();
                copy.replace_range(item.start()..item.end(), replacement);
                results.push(copy);
            }
        }
        return results;
    }

    fn search_formula(&self) -> i32 {
        let mut current = self.input.clone();
        let mut count = 0;

        while current != "e" {
            let mut found = false;
            let prev = current.clone();

            for (replace, with) in &self.replacements {
                for item in with {
                    let matched = Regex::new(item).unwrap().find(&current);

                    if matched != None {
                        let mut clone = current.clone();
                        clone.replace_range(matched.unwrap().start()..matched.unwrap().end(), replace);
                        current = clone;
                        count += 1;
                        found = true;
                        break;
                    }
                }

                if found {
                    break;
                }
            }

            if current == prev {
                panic!("try again");
            }
        }
        return count;
    }
}


impl LineMatch for MolMix {
    fn get_regex(&self) -> Regex {
        return Regex::new(r"(?P<source>\w+) => (?P<replacement>\w+)").unwrap();
    }

    fn add_match(&mut self, cap: Captures) {
        let source = String::from(cap.name("source").unwrap().as_str());
        let replacement = String::from(cap.name("replacement").unwrap().as_str());

        if self.replacements.contains_key(&source) {
            self.replacements.get_mut(&source).unwrap().push(replacement);
        } else {
            self.replacements.insert(source, vec![replacement]);
        }
    }

    fn add_line(&mut self,  line: &String) {
        let re = self.get_regex();
        match re.captures(line) {
            None => {
                if line == "" {
                    return;
                }
                self.calculate_result(line);
            },
            Some(capture) => {
                self.add_match(capture);
            }
        }
    }

}

impl New<Day19> for Day19 {
    fn new(file_name: &str) -> Day19 {
        Day19 { input: Day19::get_content(file_name)}
    }
}

impl Parts for Day19 {
    fn part1(&self) -> i32 {
        let mut mixer = MolMix::new();
        for line in &self.input {
            mixer.add_line(line);
        }

        return mixer.count_result();
    }

    fn part2(&self) -> i32 {
        let mut mixer = MolMix::new();
        for line in &self.input {
            mixer.add_line(line);
        }

        return mixer.search_formula();
    }
}