use crate::days::day::{New, Parts};
use regex::{Regex, Captures};
use std::collections::HashSet;
use itertools::Itertools;

pub struct Day9 {
    input: Vec<String>,
}

struct Route {
    pub start: String,
    pub end: String,
    pub distance: i32,
}

impl Route {
    fn new(start: &String, end: &String, distance: &i32) -> Self {
        Route {start: start.clone(), end: end.clone(), distance: distance.clone()}
    }
}

struct Router {
    all_routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router{
        Router { all_routes: Vec::new() }
    }

    pub fn add_route(&mut self, line: &String) {
        let re = Regex::new(r"^(?P<start>\w+) to (?P<end>\w+) = (?P<dist>\d+)$").unwrap();
        match re.captures(line) {
            None => panic!("No idea what to do: {}", line),
            Some(capture) => {
                self.add_match(capture);
            }
        }
    }

    pub fn get_max_distance(&self) -> i32 {
        let mut min_route = None;

        let cities = self.get_cities();
        let permutations = cities.iter().permutations(cities.len()).unique();
        for permutation in permutations {
            let mut dist = 0;
            for i in 0..permutation.len() - 1 {
                dist += self.get_distance(permutation[i], permutation[i + 1]).unwrap();
            }

            match min_route {
                Some(distance) => {
                    min_route = if distance < dist {Some(dist.clone())} else {min_route};
                },
                None => min_route = Some(dist.clone()),
            }
        }
        return min_route.unwrap();
    }

    pub fn get_min_distance(&self) -> i32 {
        let mut min_route = None;

        let cities = self.get_cities();
        let permutations = cities.iter().permutations(cities.len()).unique();
        for permutation in permutations {
            let mut dist = 0;
            for i in 0..permutation.len() - 1 {
                dist += self.get_distance(permutation[i], permutation[i + 1]).unwrap();
            }

            match min_route {
                Some(distance) => {
                    min_route = if distance > dist {Some(dist.clone())} else {min_route};
                },
                None => min_route = Some(dist.clone()),
            }
        }
        return min_route.unwrap();
    }

    fn get_distance(&self, start: &String, end: &String) -> Option<i32> {
        for route in &self.all_routes {
            if route.start == *start && route.end == *end || route.end == *start && route.start == *end {
                return Some(route.distance);
            }

        }
        return None;
    }

    fn get_cities(&self) -> HashSet<String> {
        let mut known_cities = HashSet::new();
        for route in &self.all_routes {
            known_cities.insert(route.start.clone());
            known_cities.insert(route.end.clone());
        }
        return known_cities;
    }

    fn add_match(&mut self, cap: Captures) {
        let start = String::from(cap.name("start").unwrap().as_str());
        let end = String::from(cap.name("end").unwrap().as_str());
        let distance = cap.name("dist").unwrap().as_str().parse::<i32>().unwrap();

        self.all_routes.push(Route::new(&start, &end, &distance));
        self.all_routes.push(Route::new(&end, &start, &distance));
    }
}

impl New<Day9> for Day9 {
    fn new(file_name: &str) -> Day9 {
        Day9 { input: Day9::get_content(file_name)}
    }
}

impl Parts for Day9 {
    fn part1(&self) -> i32 {
        let mut router = Router::new();
        for line in &self.input {
            router.add_route(line);
        }

        return router.get_min_distance();
    }

    fn part2(&self) -> i32 {
        let mut router = Router::new();
        for line in &self.input {
            router.add_route(line);
        }

        return router.get_max_distance();
    }
}