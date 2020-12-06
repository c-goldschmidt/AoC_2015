use crate::days::day::{New, Parts};

pub struct Day8 {
    input: Vec<String>,
}

impl New<Day8> for Day8 {
    fn new(file_name: &str) -> Day8 {
        Day8 { input: Day8::get_content(file_name)}
    }
}

impl Day8 {
    fn count_string_code(&self, line: &String) -> i32 {
        let split: Vec<&str> = line.split("").collect();
        let mut num_chars = 0;
        let mut i = 2;
        while i < line.len() {
            let char = split[i.clone()];

            if char == "\\" {
                if split[i.clone() + 1] == "x" {
                    i += 3;
                } else {
                    i += 1;
                }
            }

            num_chars += 1;
            i += 1;
        }
        return num_chars;
    }

    fn escaped_length(&self, line: &String) -> i32 {
        let mut replaced = line.replace("\\", "\\\\");
        replaced = replaced.replace("\"", "\\\"");
        replaced.len() as i32 + 2
    }
}

impl Parts for Day8 {
    fn part1(&self) -> i32 {
        let mut total = 0;
        for line in &self.input {
            let code_len = self.count_string_code(line);
            let total_len = line.len() as i32;
            total += total_len - code_len;
        }
        return total;
    }

    fn part2(&self) -> i32 {
        let mut total = 0;
        for line in &self.input {
            let line_len = line.len() as i32;
            let encoded_len = self.escaped_length(line);

            total += encoded_len - line_len;
        }
        return total;
    }
}