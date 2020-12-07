use regex::{Regex, Captures};

pub trait LineMatch {
    fn get_regex(&self) -> Regex;
    fn add_match(&mut self, cap: Captures);

    fn add_line(&mut self,  line: &String) {
        let re = self.get_regex();
        match re.captures(line) {
            None => panic!("No idea what to do: {}", line),
            Some(capture) => {
                self.add_match(capture);
            }
        }
    }
}
