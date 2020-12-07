use crate::days::day::{New, Parts};
use regex::{Captures, Regex};
use crate::days::utils::LineMatch;
use std::collections::HashMap;

pub struct Day16 {
    input: Vec<String>,
}

struct Sue {
    pub num: i32,
    known_props: HashMap<String, i32>,
}

impl Sue {
    pub fn new(cap: &Captures) -> Self {
        let mut map = HashMap::new();
        let num = cap.name("num").unwrap().as_str().parse::<i32>().unwrap();
        Sue::read_prop(cap, "key1", "val1", &mut map);
        Sue::read_prop(cap, "key2", "val2", &mut map);
        Sue::read_prop(cap, "key3", "val3", &mut map);
        Sue {num, known_props: map}
    }

    fn read_prop(cap: &Captures, key: &str, value: &str, map: &mut HashMap<String, i32>) {
        let key = String::from(cap.name(key).unwrap().as_str());
        let value = cap.name(value).unwrap().as_str().parse::<i32>().unwrap();
        map.insert(key, value);
    }

    fn num_clues(&self, clues: &HashMap<String, i32>) -> i32 {
        let mut matches = 0;
        for (key, value) in clues.into_iter() {
            if self.known_props.contains_key(key) && self.known_props.get(key).unwrap() == value {
                matches += 1;
            }
        }
        return matches;
    }

    fn num_fuzzy_clues(&self, clues: &HashMap<String, i32>) -> i32 {
        let mut matches = 0;
        for (key, value) in clues.into_iter() {
            if !self.known_props.contains_key(key) {
                continue;
            }

            let own_value = self.known_props.get(key).unwrap();
            match key.as_str() {
                "cats" | "trees" => {
                    if own_value > value {
                        matches += 1;
                    }
                }
                "pomeranians" | "goldfish" => {
                    if own_value < value {
                        matches += 1;
                    }
                }
                _ => {
                    if own_value == value {
                        matches += 1;
                    }
                }
            }
        }
        return matches;
    }
}

struct SueList {
    sues: Vec<Sue>,
}

impl SueList {
    pub fn new() -> Self {
        SueList { sues: Vec::new() }
    }

    fn best_score(&self, clues: &HashMap<String, i32>, fuzzy: bool) -> i32 {
        let mut best_match = -1;
        let mut max_match = -1;

        for sue in &self.sues {
            let sue_match = if fuzzy { sue.num_fuzzy_clues(clues)  } else { sue.num_clues(clues) } ;
            if sue_match > max_match {
                best_match = sue.num;
                max_match = sue_match;
            }
        }

        return best_match;
    }
}

impl LineMatch for SueList {
    fn get_regex(&self) -> Regex {
        Regex::new(r"^Sue (?P<num>\d+): (?P<key1>\w+): (?P<val1>\d+), (?P<key2>\w+): (?P<val2>\d+), (?P<key3>\w+): (?P<val3>\d+)$").unwrap()
    }

    fn add_match(&mut self, cap: Captures) {
        self.sues.push(Sue::new(&cap));
    }
}

impl Day16 {
    fn get_clues(&self) -> HashMap<String, i32> {
        let mut clues = HashMap::new();
        clues.insert(String::from("children"), 3);
        clues.insert(String::from("cats"), 7);
        clues.insert(String::from("samoyeds"), 2);
        clues.insert(String::from("pomeranians"), 3);
        clues.insert(String::from("akitas"), 0);
        clues.insert(String::from("vizslas"), 0);
        clues.insert(String::from("goldfish"), 5);
        clues.insert(String::from("trees"), 3);
        clues.insert(String::from("cars"), 2);
        clues.insert(String::from("perfumes"), 1);
        return clues;
    }
}

impl New<Day16> for Day16 {
    fn new(file_name: &str) -> Day16 {
        Day16 { input: Day16::get_content(file_name)}
    }
}

impl Parts for Day16 {
    fn part1(&self) -> i32 {
        let mut list = SueList::new();
        for line in &self.input {
            list.add_line(line);
        }

        return list.best_score(&self.get_clues(), false);
    }

    fn part2(&self) -> i32 {
        let mut list = SueList::new();
        for line in &self.input {
            list.add_line(line);
        }
        return list.best_score(&self.get_clues(), true);
    }
}