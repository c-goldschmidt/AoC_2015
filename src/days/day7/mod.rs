use crate::days::day::{New, Parts};

mod wireboard;


pub struct Day7 {
    input: Vec<String>,
}

impl New<Day7> for Day7 {
    fn new(file_name: &str) -> Day7 {
        Day7 { input: Day7::get_content(file_name)}
    }
}

impl Parts for Day7 {
    fn part1(&self) -> i32 {
        let mut board = wireboard::WireBoard::new();
        for line in &self.input {
            board.add_instruction(line);
        }

        return board.get_wire_value(&String::from("wire_a")) as i32;
    }

    fn part2(&self) -> i32 {
        let value1 = self.part1() as u16;

        let mut board = wireboard::WireBoard::new();
        for line in &self.input {
            board.add_instruction(line);
        }
        board.add_instruction(&format!("{} -> b", value1));

        return board.get_wire_value(&String::from("wire_a")) as i32;
    }
}