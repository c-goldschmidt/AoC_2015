use std::collections::HashMap;
use regex::{Regex, Captures};
pub enum LogicOperation { AND, OR, LSHIFT, RSHIFT, NOT }


pub struct WireBoard {
    wires: HashMap<String, Wire>,
    gates: HashMap<String, Gate>,
    constants: HashMap<String, Constant>,
    cache: HashMap<String, u16>,
}

pub struct Constant {
    pub value: u16,
}

pub struct Wire {
    pub connected: bool,
    pub source: Option<String>,
}

pub struct Gate {
    pub left: Option<String>,
    pub right: Option<String>,
    operation: LogicOperation,
}

impl Constant {
    pub fn new(value: u16) -> Self { Constant{value} }
}

impl Wire {
    pub fn new(source: Option<&String>) -> Self {
        Wire {
            source: match source {Some(x) => Some(x.to_string().clone()), None => None},
            connected: match source {Some(_) => true, None => false},
        }
    }
}

impl Gate {
    pub fn new(left: Option<String>, right: Option<String>, operation: LogicOperation) -> Self {
        Gate {left: left.clone(), right: right.clone(), operation}
    }

    pub fn calculate(&self, left: &Option<u16>, right: &Option<u16>) -> u16 {
        match self.operation {
            LogicOperation::NOT => self.not(left),
            LogicOperation::AND => self.and(left, right),
            LogicOperation::OR => self.or(left, right),
            LogicOperation::LSHIFT => self.lshift(left, right),
            LogicOperation::RSHIFT => self.rshift(left, right),
        }
    }

    fn not(&self, left: &Option<u16>) -> u16 {
        return !left.unwrap();
    }

    fn and(&self, left: &Option<u16>, right: &Option<u16>) -> u16 {
        return left.unwrap() & right.unwrap();
    }

    fn or(&self, left: &Option<u16>, right: &Option<u16>) -> u16 {
        return left.unwrap() | right.unwrap();
    }

    fn lshift(&self, left: &Option<u16>, right: &Option<u16>) -> u16 {
        return left.unwrap() << right.unwrap();
    }

    fn rshift(&self, left: &Option<u16>, right: &Option<u16>) -> u16 {
        return left.unwrap() >> right.unwrap();
    }
}

impl Clone for Gate {
    fn clone(&self) -> Self {
        Gate {
            left: self.left.clone(),
            right: self.right.clone(),
            operation: match self.operation {
                LogicOperation::AND => LogicOperation::AND,
                LogicOperation::OR => LogicOperation::OR,
                LogicOperation::NOT => LogicOperation::NOT,
                LogicOperation::LSHIFT => LogicOperation::LSHIFT,
                LogicOperation::RSHIFT => LogicOperation::RSHIFT,
            },
        }
    }
}

impl WireBoard {
    pub fn new() -> Self {
        WireBoard {wires: HashMap::new(), gates: HashMap::new(), constants: HashMap::new(), cache: HashMap::new()}
    }

    pub fn get_wire_value(&mut self, wire_name: &String) -> u16 {
        self.resolve(wire_name).unwrap()
    }

    pub fn resolve(&mut self, name: &String) -> Option<u16> {
        if !self.cache.contains_key(name) {
            let value = self.calculate_node(name);
            self.cache.insert(name.clone(), value.unwrap());
        }

        match self.cache.get(name) {
            Some(x) => Some(*x),
            None => None
        }
    }

    fn calculate_node(&mut self, name: &String) -> Option<u16> {
            let name_split: Vec<&str> = name.split("_").collect();
            match name_split[0] {
                "wire" => self.calculate_wire(name),
                "gate" =>  self.calculate_gate(name),
                "constant" =>  match self.constants.get(name) {
                    Some(constant) => Some(constant.value),
                    None => None,
                },
                _ => None
            }
    }

    fn calculate_wire(&mut self, name: &String) -> Option<u16>{
        let source = self.wires.get_mut(name).unwrap().source.as_ref().unwrap().clone();
        self.resolve(&source)
    }

    fn calculate_gate(&mut self, name: &String) -> Option<u16>{
        let gate = self.gates.get_mut(name).unwrap().clone();
        let left_name = gate.left.as_ref();
        let right_name = gate.right.as_ref();

        let left = self.resolve(&left_name.unwrap()).clone();
        let right = match right_name {
            Some(name) => self.resolve(name),
            None=> None
        } ;

        Some(gate.calculate(&left, &right))
    }

    pub fn add_instruction(&mut self, line: &String) {
        let re = Regex::new(r"^(?:(?P<input1>(?:\w{1,2}|\d+)) )?(?:(?P<action>(?:NOT|OR|AND|LSHIFT|RSHIFT)) )?(?:(?P<input2>\w+) )?-> (?P<output>\w+)").unwrap();
        let matched = re.captures(line);
        match matched {
            None => panic!("No idea what to do: {}", line),
            Some(matched) => self.add_match(matched),
        }
    }

    fn add_match(&mut self, cap: Captures) {
        // println!("{} => match", &cap[0]);
        match cap.name("action") {
            Some(action) => match action.as_str() {
                "AND" => self.add_and(cap),
                "OR" => self.add_or(cap),
                "RSHIFT" => self.add_rshift(cap),
                "LSHIFT" => self.add_lshift(cap),
                "NOT" => self.add_not(cap),
                _ => self.add_constant_or_wire(cap),
            },
            None => {
                self.add_constant_or_wire(cap);
            },
        }
    }

    fn add_gate(&mut self, cap: Captures, operation: LogicOperation) {
        let source1 = self.get_or_create_source(&String::from(cap.name("input1").unwrap().as_str()));
        let source2 = self.get_or_create_source(&String::from(cap.name("input2").unwrap().as_str()));
        let gate_name = String::from("gate_".to_string() + &cap[0].to_string());
        self.create_or_update_wire(&String::from(cap.name("output").unwrap().as_str()), Some(&gate_name));
        self.gates.insert(gate_name, Gate::new(source1, source2, operation));
    }

    fn add_and(&mut self, cap: Captures) {
        self.add_gate(cap, LogicOperation::AND);
    }

    fn add_or(&mut self, cap: Captures) {
        self.add_gate(cap, LogicOperation::OR);
    }

    fn add_rshift(&mut self, cap: Captures) {
        self.add_gate(cap, LogicOperation::RSHIFT);
    }

    fn add_lshift(&mut self, cap: Captures) {
        self.add_gate(cap, LogicOperation::LSHIFT);
    }

    fn add_not(&mut self, cap: Captures) {
        let source = self.get_or_create_source(&String::from(cap.name("input2").unwrap().as_str()));
        let gate_name = String::from("gate_".to_string() + &cap[0].to_string());
        self.create_or_update_wire(&String::from(cap.name("output").unwrap().as_str()), Some(&gate_name));
        self.gates.insert(gate_name, Gate::new(source, None, LogicOperation::NOT));
    }

    fn add_constant_or_wire(&mut self, cap: Captures) {
        match self.get_or_create_source(&String::from(cap.name("input1").unwrap().as_str())) {
            Some(name) => {
                self.create_or_update_wire(&String::from(cap.name("output").unwrap().as_str()), Some(&name));
            },
            None => {},
        };
    }

    fn get_or_create_source(&mut self, source_name: &String) -> Option<String> {
        match source_name.parse::<u16>() {
            Ok(value) => {
                let source_name = "constant_".to_string() + &source_name.to_string();
                self.constants.insert(source_name.clone(), Constant::new(value));
                Some(source_name)
            },
            Err(_) => {
                self.create_or_update_wire(source_name, None)
            }
        }
    }

    fn create_or_update_wire(&mut self, wire_name: &String, from: Option<&String>) -> Option<String> {
        let wire_name = "wire_".to_string() + &wire_name.to_string();

        match from {
            Some(_) => {
                if self.wires.contains_key(&wire_name) {
                    if self.wires.get(&wire_name).unwrap().connected {
                        println!("reconnecting already connected wire (!)");
                    }
                }
                self.wires.insert(wire_name.clone(), Wire::new(from.clone()));
            },
            None => {
                // don't disconnect already connected wires
                if !self.wires.contains_key(&wire_name) {
                    self.wires.insert(wire_name.clone(), Wire::new(from.clone()));
                }
            }
        };

        return Some(wire_name);
    }
}