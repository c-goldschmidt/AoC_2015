use crate::days::day::{New, Parts};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use itertools::Itertools;

pub struct Day20 {
    input: Vec<String>,
}

impl New<Day20> for Day20 {
    fn new(file_name: &str) -> Day20 {
        Day20 { input: Day20::get_content(file_name)}
    }
}

impl Day20 {
    fn calc_chunked(&self) -> i32 {
        let search = self.input[0].parse::<i32>().unwrap();
        let (tx, rx) = channel();
        let pool = ThreadPool::new(8);

        for chunk in &(1..i32::MAX).into_iter().chunks(8) {
            for house in chunk.collect::<Vec<i32>>().into_iter() {

                let tx = tx.clone();
                pool.execute(move|| {
                    let value =  (1..house + 1).into_iter().map(move|elf| ((house % elf == 0) as i32 * elf) * 10).sum::<i32>();
                    match tx.send((house, value)) { _ => {}};
                });
            }

            for (house, value) in rx.iter().take(8) {
                if value >= search {
                    return house;
                }
            }
        }

        return -1;
    }

    fn calc_chunked_limited(&self) -> i32 {
        let search = self.input[0].parse::<i32>().unwrap();
        let (tx, rx) = channel();
        let pool = ThreadPool::new(8);

        for chunk in &(1..i32::MAX).into_iter().chunks(8) {
            for house in chunk.collect::<Vec<i32>>().into_iter() {

                let tx = tx.clone();
                pool.execute(move|| {
                    let value =  (1..house + 1).into_iter().map(move|elf| {
                        return ((house % elf == 0 && (house as f32 / elf as f32 <= 50 as f32)) as i32 * elf) * 11
                    }).sum::<i32>();
                    match tx.send((house, value)) { _ => {}};
                });
            }

            for (house, value) in rx.iter().take(8) {
                if value >= search {
                    return house;
                }
            }
        }

        return -1;
    }
}

impl Parts for Day20 {
    fn part1(&self) -> i32 {
        return self.calc_chunked();
    }

    fn part2(&self) -> i32 {
        return self.calc_chunked_limited();
    }
}