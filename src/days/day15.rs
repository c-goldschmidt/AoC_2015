use crate::days::day::{New, Parts};
use crate::days::utils::LineMatch;
use regex::{Regex, Captures};
use itertools::Itertools;

pub struct Day15 {
    input: Vec<String>,
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    pub fn new(capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Ingredient { capacity, durability, flavor, texture, calories }
    }

    pub fn empty() -> Self {
        Ingredient { capacity: 0, durability: 0, flavor: 0, texture: 0, calories: 0 }
    }

    pub fn mix(ingredients: &Vec<Self>, counts: &Vec<i32>) -> Self {
        assert_eq!(ingredients.len(),  counts.len());

        let mut mixed = Ingredient::empty();

        for i in 0..ingredients.len() {
            mixed.capacity += counts[i] * ingredients[i].capacity;
            mixed.durability += counts[i] * ingredients[i].durability;
            mixed.flavor += counts[i] * ingredients[i].flavor;
            mixed.texture += counts[i] * ingredients[i].texture;
            mixed.calories += counts[i] * ingredients[i].calories;
        }

        if mixed.capacity < 0 {
            mixed.capacity = 0;
        }

        if mixed.durability < 0 {
            mixed.durability = 0;
        }

        if mixed.flavor < 0 {
            mixed.flavor = 0;
        }

        if mixed.texture < 0 {
            mixed.texture = 0;
        }

        if mixed.calories < 0 {
            mixed.calories = 0;
        }

        return mixed;
    }

    fn score(&self) -> i32 {
        return (self.capacity as i64 * self.durability as i64 * self.flavor as i64 * self.texture as i64) as i32;
    }
}

struct Kitchen {
    ingredients: Vec<Ingredient>,
}

impl Kitchen {
    fn new() -> Self {
        Kitchen { ingredients: Vec::new() }
    }

    fn best_score(&self, diet: bool) -> i32 {
        // this is suuuuuuper slow, because i'm throwing away about 99% of the generated permutations
        // but...i don't have a better idea right now
        let permutations = (1..(101 - self.ingredients.len() as i32)).permutations(self.ingredients.len()).filter(|x| (x.clone().into_iter()).sum1::<i32>().unwrap() == 100).unique();

        let mut score = 0;

        for permutation in permutations {
            let cookie = Ingredient::mix(&self.ingredients, &permutation);
            let cookie_score = cookie.score();
            if (!diet || cookie.calories == 500) && cookie_score > score {
                score = cookie_score;
            }
        }

        return score;
    }
}

impl LineMatch for Kitchen {
    fn get_regex(&self) -> Regex {
        Regex::new(r"^(?P<name>\w+).+?(?P<capacity>-?\d+).+?(?P<durability>-?\d+).+?(?P<flavor>-?\d+).+?(?P<texture>-?\d+).+?(?P<calories>-?\d+)$").unwrap()
    }

    fn add_match(&mut self, cap: Captures) {
        let capacity = cap.name("capacity").unwrap().as_str().parse::<i32>().unwrap();
        let durability = cap.name("durability").unwrap().as_str().parse::<i32>().unwrap();
        let flavor = cap.name("flavor").unwrap().as_str().parse::<i32>().unwrap();
        let texture = cap.name("texture").unwrap().as_str().parse::<i32>().unwrap();
        let calories = cap.name("calories").unwrap().as_str().parse::<i32>().unwrap();
        self.ingredients.push(Ingredient::new(capacity, durability, flavor, texture, calories));
    }
}


impl New<Day15> for Day15 {
    fn new(file_name: &str) -> Day15 {
        Day15 { input: Day15::get_content(file_name)}
    }
}

impl Parts for Day15 {
    fn part1(&self) -> i32 {
        let mut kitchen = Kitchen::new();
        for line in &self.input {
            kitchen.add_line(line);
        }

        return kitchen.best_score(false);
    }

    fn part2(&self) -> i32 {
        let mut kitchen = Kitchen::new();
        for line in &self.input {
            kitchen.add_line(line);
        }

        return kitchen.best_score(true);
    }
}