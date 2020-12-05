use crate::days::day::{New, Parts};
use std::cmp;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (cmp::min($x, min!($($z),*)));
}

pub struct Day2 {
    input: Vec<String>,
}

impl New<Day2> for Day2 {
    fn new(file_name: &str) -> Day2 {
        Day2 { input: Day2::get_content(file_name)}
    }
}

impl Parts for Day2 {
    fn part1(&self) -> i32 {
        let mut total = 0;
        for row in self.input.clone() {
            let dimensions: Vec<i32> = row.split("x").map(|x| x.parse::<i32>().unwrap()).collect();

            let lw =  2 * dimensions[0] * dimensions[1];
            let wh =  2 * dimensions[1] * dimensions[2];
            let hl =  2 * dimensions[2] * dimensions[0];

            total += lw + wh + hl + (min!(lw, wh, hl) / 2) ;
        }

        return total;
    }

    fn part2(&self) -> i32 {
        let mut total = 0;

        for row in self.input.clone() {
            let mut dimensions: Vec<i32> = row.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
            dimensions.sort();

            total += 2 * dimensions[0];
            total += 2 * dimensions[1];
            total += dimensions[0] * dimensions[1] * dimensions[2];

        }

        return total;
    }
}