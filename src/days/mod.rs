mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        _ => panic!("Day {} not implemented!", day_num),
    }
}