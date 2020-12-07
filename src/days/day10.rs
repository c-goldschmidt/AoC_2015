use crate::days::day::{New, Parts};

pub struct Day10 {
    input: Vec<String>,
}

impl New<Day10> for Day10 {
    fn new(file_name: &str) -> Day10 {
        Day10 { input: Day10::get_content(file_name)}
    }
}

impl Day10 {

    fn encoded_length(&self, input: &String) -> String {
        let split: Vec<&str> = input.split("").collect();

        let mut count = 0;
        let mut prev = None;
        let mut output = String::from("");

        for i in 1..input.len() + 1 {
            let current = split[i];
            match prev {
                Some(value) => {
                    if value != current {
                        output += &format!("{}{}", count, value);
                        count = 0;
                    }
                },
                None => {},
            };
            prev = Some(current);
            count += 1;
        }

        match prev {
            Some(value) => output += &format!("{}{}", count, value),
            None => {},
        }

        return output;
    }

}

impl Parts for Day10 {
    fn part1(&self) -> i32 {
        let mut result = self.encoded_length(&self.input[0]);
        for _i in 0..39 {
            result = self.encoded_length(&result);
        }
        return result.len() as i32;
    }

    fn part2(&self) -> i32 {
        let mut result = self.encoded_length(&self.input[0]);
        for _i in 0..49 {
            result = self.encoded_length(&result);
        }
        return result.len() as i32;
    }
}