use crate::days::day::{New, Parts};
use regex::{Captures, Regex};
use std::collections::HashSet;
use itertools::Itertools;
use crate::days::utils::LineMatch;

pub struct Day13 {
    input: Vec<String>,
}

struct HappyEntry {
    pub left: String,
    pub right: String,
    pub modifier: i32,
}

impl HappyEntry {
    pub fn new(left: &String, right: &String, modifier: i32) -> Self {
        HappyEntry {left: left.clone(), right: right.clone(), modifier}
    }
}

struct HappyMeter {
    items: Vec<HappyEntry>,
    all_members: HashSet<String>,
}

impl HappyMeter {
    pub fn new() -> Self {
        HappyMeter { items: Vec::new(), all_members: HashSet::new() }
    }

    pub fn add_self(&mut self) {
        let myself = String::from("Myself");
        for member in &self.all_members {

            self.items.push(HappyEntry::new(member, &myself, 0));
            self.items.push(HappyEntry::new(&myself, member, 0));
        }

        self.all_members.insert(myself);
    }

    pub fn get_best_score(&self) -> i32 {
        let permutations = self.all_members.iter().permutations(self.all_members.len()).unique();
        let mut best = None;

        for permutation in permutations {
            let perm_len = permutation.len();
            let mut perm_score = 0;
            for i in 0..perm_len - 1 {
                perm_score += self.get_score(permutation[i], permutation[i + 1]);
            }

            // last next to first (round table)
            perm_score += self.get_score(permutation[perm_len - 1], permutation[0]);
            match best {
                Some(score) => {
                    if score > perm_score {
                        perm_score = score;
                    }
                },
                None => {},
            };
            best = Some(perm_score);
        }

        match best {
            Some(value) => value,
            None => {
                panic!("did not find any seatings (?)");
            }
        }
    }

    fn get_score(&self, left: &String, right: &String) -> i32 {
        let mut total = 0;
        for item in &self.items {
            if (item.left == *left && item.right == *right) || (item.right == *left && item.left == *right)  {
                total += item.modifier;
            }
        }
        return total;
    }

}

impl LineMatch for HappyMeter {
    fn get_regex(&self) -> Regex {
        Regex::new(r"^(?P<left>\w+) would (?P<op>\w+) (?P<value>\d+).+?(?P<right>\w+)\.$").unwrap()
    }

    fn add_match(&mut self, cap: Captures) {
        let left = String::from(cap.name("left").unwrap().as_str());
        let right = String::from(cap.name("right").unwrap().as_str());
        let op = if cap.name("op").unwrap().as_str() == "gain" { 1 } else { -1 };
        let value = cap.name("value").unwrap().as_str().parse::<i32>().unwrap() * op;

        self.items.push(HappyEntry::new(&left, &right, value));
        self.all_members.insert(left);
        self.all_members.insert(right);
    }
}

impl New<Day13> for Day13 {
    fn new(file_name: &str) -> Day13 {
        Day13 { input: Day13::get_content(file_name)}
    }
}

impl Parts for Day13 {
    fn part1(&self) -> i32 {
        let mut happy_meter = HappyMeter::new();
        for line in &self.input {
            happy_meter.add_line(line);
        }

        return happy_meter.get_best_score();
    }

    fn part2(&self) -> i32 {
        let mut happy_meter = HappyMeter::new();
        for line in &self.input {
            happy_meter.add_line(line);
        }

        happy_meter.add_self();
        return happy_meter.get_best_score();
    }
}