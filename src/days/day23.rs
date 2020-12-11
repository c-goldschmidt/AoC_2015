use crate::days::day::{New, Parts};
use crate::days::utils::LineMatch;
use regex::{Regex, Captures};

#[derive(PartialEq,Copy,Clone)]
enum Action { HLF, TPL, INC, JMP, JIE, JIO }

#[derive(PartialEq,Copy,Clone)]
enum Register { A, B }

pub struct Day23 {
    input: Vec<String>,
}

struct Instruction {
    action: Action,
    register: Option<Register>,
    value: Option<i32>,
}

impl Instruction {
    fn new(action: Action, register: Option<Register>, value: Option<i32>) -> Self {
        Instruction { action, register, value }
    }
}

struct StateMachine {
    instructions: Vec<Instruction>,
    pointer: usize,
    reg_a: i32,
    reg_b: i32,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine { instructions: Vec::new(), pointer: 0, reg_a: 0, reg_b: 0 }
    }

    pub fn exec(&mut self) {
        let instruction = &self.instructions[self.pointer.clone()];

        match instruction.action {
            Action::HLF => match instruction.register.unwrap() {
                Register::A => { self.reg_a /= 2 },
                Register::B => { self.reg_b /= 2 },
            },
            Action::TPL => match instruction.register.unwrap() {
                Register::A => { self.reg_a *= 3 },
                Register::B => { self.reg_b *= 3 },
            },
            Action::INC => match instruction.register.unwrap() {
                Register::A => { self.reg_a += 1 },
                Register::B => { self.reg_b += 1 },
            },
            Action::JMP => {
                self.pointer = (self.pointer as i32 + instruction.value.unwrap() - 1) as usize;
            },
            Action::JIE => {
                let val = match instruction.register.unwrap() {
                    Register::A => self.reg_a,
                    Register::B => self.reg_b,
                };

                if val % 2 == 0 {
                    self.pointer = (self.pointer as i32 + instruction.value.unwrap() - 1) as usize;
                }
            },
            Action::JIO => {
                let val = match instruction.register.unwrap() {
                    Register::A => self.reg_a,
                    Register::B => self.reg_b,
                };
                if val == 1 {
                    self.pointer = (self.pointer as i32 + instruction.value.unwrap() - 1) as usize;
                }
            },
        }
    }

    pub fn run(&mut self) -> i32 {
        while self.pointer < (&self.instructions).len() {
            self.exec();
            self.pointer += 1;
        }

        return self.reg_b;
    }
}

impl LineMatch for StateMachine {
    fn get_regex(&self) -> Regex {
        return Regex::new(r"^(?P<instruction>\w{3}) (?:(?P<register>[ab])(?:$|, ))?(?:\+?(?P<value>-?\d+))?$").unwrap();
    }

    fn add_match(&mut self, cap: Captures) {
        let instruction = match cap.name("instruction").unwrap().as_str() {
             "hlf" => Action::HLF,
             "tpl" => Action::TPL,
             "inc" => Action::INC,
             "jmp" => Action::JMP,
             "jie" => Action::JIE,
             "jio" => Action::JIO,
            _ => panic!("Unknown action"),
        };
        let register = match cap.name("register") {
            Some(reg) => match reg.as_str() {
                "a" => Some(Register::A),
                "b" => Some(Register::B),
                _ => None,
            },
            None => None,
        };
        let value = match cap.name("value") {
            Some(val) => Some(val.as_str().parse::<i32>().unwrap()),
            None => None
        };

        self.instructions.push(Instruction::new(instruction, register, value));
    }
}

impl New<Day23> for Day23 {
    fn new(file_name: &str) -> Day23 {
        Day23 { input: Day23::get_content(file_name)}
    }
}

impl Parts for Day23 {
    fn part1(&self) -> i32 {
        let mut machine = StateMachine::new();
        for line in &self.input {
            machine.add_line(line);
        }

        return machine.run();
    }

    fn part2(&self) -> i32 {
        let mut machine = StateMachine::new();
        for line in &self.input {
            machine.add_line(line);
        }
        machine.reg_a = 1;
        return machine.run();
    }
}