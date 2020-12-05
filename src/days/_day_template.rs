use crate::days::day::{New, Parts};

pub struct DayX {
    input: Vec<String>,
}

impl New<DayX> for DayX {
    fn new(file_name: &str) -> DayX {
        DayX { input: DayX::get_content(file_name)}
    }
}

impl Parts for DayX {
    fn part1(&self) -> i32 {
        return 0;
    }

    fn part2(&self) -> i32 {
        return 0;
    }
}