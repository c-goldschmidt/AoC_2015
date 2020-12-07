use crate::days::day::{New, Parts};
use regex::{Captures, Regex};
use crate::days::utils::LineMatch;

pub struct Day14 {
    input: Vec<String>,
}

struct Reindeer {
    name: String,
    speed: i32,
    endurance: i32,
    rest: i32,
    distance: i32,
    rest_remain: i32,
    points: i32,
    endurance_remain: i32,
}

impl Reindeer {
    pub fn new(name: &String, speed: i32, endurance: i32, rest: i32) -> Self {
        Reindeer {
            name: name.clone(),
            speed, endurance: endurance.clone(), rest,
            distance: 0,
            rest_remain: 0,
            points: 0,
            endurance_remain: endurance,
        }
    }

    pub fn next(&mut self) {
        if self.rest_remain > 0 {
            self.rest_remain -= 1;

            if self.rest_remain == 0 {
               self.endurance_remain = self.endurance;
            }
            return;
        }

        if self.endurance_remain > 0 {
            self.endurance_remain -= 1;
            self.distance += self.speed;
        } else {
            // first second of rest
            self.rest_remain = self.rest - 1;
        }
    }
}

struct Race {
    participants: Vec<Reindeer>,
}

impl Race {
    pub fn new() -> Self {
        Race { participants: Vec::new() }
    }

    pub fn run_for_distance(&mut self, seconds: i32) -> i32 {
        for _i in 0..seconds {
            for reindeer in &mut self.participants {
                reindeer.next();
            }
        }

        self.participants.sort_by(|a, b| b.distance.cmp(&a.distance));
        println!(
            "{} wins @ {} (followed by {} @ {})",
            self.participants[0].name,
            self.participants[0].distance,
            self.participants[1].name,
            self.participants[1].distance,
        );
        return self.participants[0].distance;
    }

    pub fn run_for_points(&mut self, seconds: i32) -> i32 {
        for _i in 0..seconds {
            for reindeer in &mut self.participants {
                reindeer.next();
            }
            self.update_lead();
        }

        self.participants.sort_by(|a, b| b.points.cmp(&a.points));
        println!(
            "{} wins @ {} (followed by {} @ {})",
            self.participants[0].name,
            self.participants[0].points,
            self.participants[1].name,
            self.participants[1].points,
        );
        return self.participants[0].points;
    }

    fn update_lead(&mut self) {
        self.participants.sort_by(|a, b| b.distance.cmp(&a.distance));
        let lead = self.participants[0].distance.clone();

        for reindeer in &mut self.participants {
            if reindeer.distance != lead {
                break;
            }
            reindeer.points += 1;
        }
    }

}

impl LineMatch for Race {
    fn get_regex(&self) -> Regex {
        Regex::new(r"^(?P<name>\w+).+?(?P<speed>\d+).+?(?P<endurance>\d+).+?(?P<rest>\d+).+?$").unwrap()
    }

    fn add_match(&mut self, cap: Captures) {
        let name = String::from(cap.name("name").unwrap().as_str());
        let speed = cap.name("speed").unwrap().as_str().parse::<i32>().unwrap();
        let endurance = cap.name("endurance").unwrap().as_str().parse::<i32>().unwrap();
        let rest = cap.name("rest").unwrap().as_str().parse::<i32>().unwrap();

        self.participants.push(Reindeer::new(&name, speed, endurance, rest));
    }
}

impl New<Day14> for Day14 {
    fn new(file_name: &str) -> Day14 {
        Day14 { input: Day14::get_content(file_name)}
    }
}

impl Parts for Day14 {
    fn part1(&self) -> i32 {
        let mut race = Race::new();
        for line in &self.input {
            race.add_line(line);
        }

        return race.run_for_distance(2503);
    }

    fn part2(&self) -> i32 {
        let mut race = Race::new();
        for line in &self.input {
            race.add_line(line);
        }

        return race.run_for_points(2503);
    }
}