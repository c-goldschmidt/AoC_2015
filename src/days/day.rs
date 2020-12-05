use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::SystemTime;

pub trait New<T> {
    fn get_content(file_name: &str) -> Vec<String> {
        let reader = BufReader::new(File::open(file_name).expect("Cannot open file"));
        let mut vec = Vec::new();
        for line in reader.lines() {
            vec.push(line.unwrap())
        }
        return vec;
    }

    fn new(file_name: &str) -> T;
}

pub trait Parts {
    fn run_both(&self, day: i32) {
        self.time_part(day, 1);
        self.time_part(day, 2);
    }

    fn time_part(&self, day: i32, part: i32) {
        let now = SystemTime::now();

        let result = match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => -1,
        };

        println!(
            "[{:.4}s] Day{} Part{} result: {}",
            now.elapsed().unwrap().as_millis() as f32 / 1000.0,
            day, part, result);
    }

    fn part1(&self) -> i32;
    fn part2(&self) -> i32;
}
