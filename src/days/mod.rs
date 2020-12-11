mod day;
mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crate::days::day::{New, Parts};

pub fn run(day_num: i32) {
    let file_name = format!("inputs/day{}.txt", day_num).to_string();
    match day_num {
        1 => day1::Day1::new(&file_name).run_both(day_num),
        2 => day2::Day2::new(&file_name).run_both(day_num),
        3 => day3::Day3::new(&file_name).run_both(day_num),
        4 => day4::Day4::new(&file_name).run_both(day_num),
        5 => day5::Day5::new(&file_name).run_both(day_num),
        6 => day6::Day6::new(&file_name).run_both(day_num),
        7 => day7::Day7::new(&file_name).run_both(day_num),
        8 => day8::Day8::new(&file_name).run_both(day_num),
        9 => day9::Day9::new(&file_name).run_both(day_num),
        10 => day10::Day10::new(&file_name).run_both(day_num),
        11 => day11::Day11::new(&file_name).run_both(day_num),
        12 => day12::Day12::new(&file_name).run_both(day_num),
        13 => day13::Day13::new(&file_name).run_both(day_num),
        14 => day14::Day14::new(&file_name).run_both(day_num),
        15 => day15::Day15::new(&file_name).run_both(day_num),
        16 => day16::Day16::new(&file_name).run_both(day_num),
        17 => day17::Day17::new(&file_name).run_both(day_num),
        18 => day18::Day18::new(&file_name).run_both(day_num),
        19 => day19::Day19::new(&file_name).run_both(day_num),
        20 => day20::Day20::new(&file_name).run_both(day_num),
        21 => day21::Day21::new(&file_name).run_both(day_num),
        22 => day22::Day22::new(&file_name).run_both(day_num),
        23 => day23::Day23::new(&file_name).run_both(day_num),
        24 => day24::Day24::new(&file_name).run_both(day_num),
        25 => day25::Day25::new(&file_name).run_both(day_num),
        _ => panic!("Day {} not implemented!", day_num),
    }
}