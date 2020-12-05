mod days;

extern crate crypto;
extern crate regex;

use std::env;
use crate::days::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    run(day);
}
