use crate::days::day::{New, Parts};
use std::collections::HashSet;

pub struct Day24 {
    input: Vec<String>,
}

struct Entangler {
    inputs: Vec<u64>,
}

impl Entangler {
    pub fn new(inputs: &Vec<String>) -> Self {
        Entangler { inputs: inputs.into_iter().map(|s| s.parse().unwrap()).collect() }
    }

    pub fn run_recurse(&self, bucket_sum: u64, min_len: usize, vec: &Vec<u64>, vec_mul: u64, vec_sum: u64, candidates: &mut HashSet<Vec<u64>>) -> u64 {
        let mut min_mul = u64::MAX;
        for next in &self.inputs {
            if vec.contains(next) {
                continue;
            }
            let mut curr_mul = vec_mul * next;
            if curr_mul > min_mul  {
                // no need to check this, already bigger
               continue;
            }

            let curr_sum = vec_sum + next;
            if curr_sum == bucket_sum {
                // valid, there can also be no other "next" that could lead to the correct sum.
                // println!("found: {}", curr_mul);
                return curr_mul;
            } else if curr_sum > bucket_sum {
                // dead end
                return u64::MAX;
            }

            // try one element more
            if vec.len() < min_len {
                let mut sub_vec = vec.clone();
                sub_vec.push(next.clone());
                sub_vec.sort();

                if candidates.contains(&sub_vec) {
                    continue;
                }
                candidates.insert(sub_vec.clone());

                curr_mul = self.run_recurse(bucket_sum, min_len, &sub_vec, curr_mul, curr_sum, candidates);

                if curr_mul < min_mul {
                    min_mul = curr_mul;
                }
            }
        }

        return min_mul;
    }

    pub fn run(&mut self, num_groups: u64) -> u64 {
        self.inputs.reverse();

        let bucket_sum = (&self.inputs).into_iter().sum::<u64>() / num_groups;
        let mut min_bucket_size = 0;
        let mut current_sum = 0;
        let in_len = (&self.inputs).len();

        for i in 0..in_len {
            let next = (&self.inputs)[i].clone();
            if current_sum + next > bucket_sum {
                continue;
            }
            min_bucket_size += 1;
            current_sum += next;
        }
        let mut candidates = HashSet::new();
        println!("sum: {}, min_size: {}", bucket_sum, min_bucket_size);
        let result = self.run_recurse(bucket_sum, min_bucket_size, &Vec::new(), 1, 0, &mut candidates);
        println!("result: {}", result);
        return result;
    }
}

impl New<Day24> for Day24 {
    fn new(file_name: &str) -> Day24 {
        Day24 { input: Day24::get_content(file_name)}
    }
}

impl Parts for Day24 {
    fn part1(&self) -> i32 {
        let mut entangler = Entangler::new(&self.input);
        return entangler.run(3) as i32;
    }

    fn part2(&self) -> i32 {
        let mut entangler = Entangler::new(&self.input);
        return entangler.run(4) as i32;
    }
}