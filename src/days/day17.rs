use crate::days::day::{New, Parts};
use itertools::Itertools;

pub struct Day17 {
    input: Vec<String>,
}

impl New<Day17> for Day17 {
    fn new(file_name: &str) -> Day17 {
        Day17 { input: Day17::get_content(file_name)}
    }
}

impl Day17 {

    fn get_containers(&self) -> Vec<i32> {
        let mut containers = Vec::new();
        for line in &self.input {
            containers.push(line.parse::<i32>().unwrap())
        }
        return containers;
    }

    fn get_num_combinations(&self, amount: i32) -> i32 {
        let containers = self.get_containers();
        let mut total_permutations = 0;

        for i in 1..containers.len() + 1 {
            let permutations = (&containers).into_iter().combinations(i).filter(|x| (x.clone().into_iter()).sum1::<i32>().unwrap() == amount);
            total_permutations += permutations.count() as i32;
        }

        return total_permutations;
    }

    fn get_num_min_combinations(&self, amount: i32) -> i32 {
        let containers = self.get_containers();

        for i in 1..containers.len() + 1 {
            let permutations = (&containers).into_iter().combinations(i).filter(|x| (x.clone().into_iter()).sum1::<i32>().unwrap() == amount);
            let count = permutations.count();
            if count > 0 {
                return count as i32;
            }
        }

        return -1;
    }
}

impl Parts for Day17 {
    fn part1(&self) -> i32 {
        return self.get_num_combinations(150);
    }

    fn part2(&self) -> i32 {
        return self.get_num_min_combinations(150);
    }
}