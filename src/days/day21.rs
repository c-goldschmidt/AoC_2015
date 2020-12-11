use crate::days::day::{New, Parts};
use crate::days::utils::LineMatch;
use regex::{Regex, Captures};
use itertools::{Itertools, Combinations};

enum Slot { WEAPON, ARMOR, RING }

pub struct Day21 {
    input: Vec<String>,
}

struct Item {
    armor: i32,
    damage: i32,
    cost: i32,
    slot: Slot,
    name: String,
}

impl Item {
    pub fn weapon(name: String, cost: i32, damage: i32) -> Self {
        Item { name, armor: 0, damage, cost, slot: Slot::WEAPON }
    }

    pub fn armor(name: String, cost: i32, armor: i32) -> Self {
        Item { name, armor, damage: 0, cost, slot: Slot::ARMOR }
    }

    pub fn ring(name: String, cost: i32, armor: i32, damage: i32) -> Self {
        Item { name, armor, damage, cost, slot: Slot::RING }
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Item { name: self.name.clone(), armor: self.armor, damage: self.damage, cost: self.cost, slot: match self.slot {
            Slot::WEAPON => Slot::WEAPON,
            Slot::ARMOR => Slot::ARMOR,
            Slot::RING => Slot::RING,
        }}
    }
}

struct Player {
    weapon: Option<Item>,
    armor: Option<Item>,
    ring1: Option<Item>,
    ring2: Option<Item>,
    hp: i32,
}

impl Player {
    pub fn new() -> Self {
        Player { weapon: None, armor: None, ring1: None, ring2: None, hp: 100 }
    }

    pub fn get_items(&self) -> Vec<&Item> {
        return vec![&self.armor, &self.weapon, &self.ring1, &self.ring2].into_iter().filter(|item| match item {
            Some(_) => true,
            None => false,
        }).map(|item| item.as_ref().unwrap()).collect();
    }

    pub fn get_attack(&self) -> i32 {
        return self.get_items().into_iter().map(|item| item.damage).sum();
    }

    pub fn get_armor(&self) -> i32 {
        return self.get_items().into_iter().map(|item| item.armor).sum();
    }

    pub fn get_gold_spent(&self) -> i32 {
        return self.get_items().into_iter().map(|item| item.cost).sum();
    }

    /*
    pub fn print_items(&self) {
        println!("{:?}", self.get_items().into_iter().map(|item| item.name.clone()).collect::<Vec<String>>());
    }
    */

    pub fn buy(&mut self, item_opt: &Option<Item>) {
        match item_opt {
            Some(item) => match item.slot {
                Slot::WEAPON => self.weapon = Some(item.clone()),
                Slot::ARMOR => self.armor = Some(item.clone()),
                Slot::RING => match self.ring1 {
                    Some(_) => { self.ring2 = Some(item.clone()); }
                    None => self.ring1 = Some(item.clone()),
                }
            },
            None=> {},
        }
    }
}

#[derive(Copy, Clone)]
struct Boss {
    damage: i32,
    hp: i32,
    armor: i32,
}

impl Boss {
    pub fn new() -> Self {
        Boss { damage: 0, hp: 0, armor: 0 }
    }
}

struct Shop {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

impl Shop {
    pub fn new() -> Self {
        Shop {
            weapons: vec![
                // weapons
                Item::weapon("Dagger".to_string(),8, 4),
                Item::weapon("Shortsword".to_string(),10, 5),
                Item::weapon("Warhammer".to_string(),25, 6),
                Item::weapon("Longsword".to_string(),40, 7),
                Item::weapon("Greataxe".to_string(),74, 8),
            ],
            armor: vec![
                Item::armor("Leather".to_string(),13, 1),
                Item::armor("Chainmail".to_string(),31, 2),
                Item::armor("Splintmail".to_string(),53, 3),
                Item::armor("Bandedmail".to_string(),75, 4),
                Item::armor("Platemail".to_string(),102, 5),
            ],
            rings: vec![
                Item::ring("Damage +1".to_string(),25, 0, 1),
                Item::ring("Damage +2".to_string(),50, 0, 2),
                Item::ring("Damage +3".to_string(),100, 0, 3),
                Item::ring("Defense +1".to_string(),20, 1, 0),
                Item::ring("Defense +1".to_string(),40, 2, 0),
                Item::ring("Defense +1".to_string(),80, 3, 0),
            ],
        }
    }
}

struct Game {
    boss: Boss,
}

impl Game {
    pub fn new() -> Game {
        Game { boss: Boss::new() }
    }

    pub fn fight(&self, player: &Player) -> bool {
        let mut boss = self.boss.clone();

        let player_atk = player.get_attack();
        let player_arm = player.get_armor();
        let mut player_hp = player.hp;

        for _round in 0..i32::MAX {
            boss.hp -= vec![player_atk - boss.armor, 1].into_iter().max().unwrap();
            if boss.hp <= 0 {
                return true
            }

            player_hp -= vec![boss.damage - player_arm, 1].into_iter().max().unwrap();
            if player_hp <= 0 {
                return false;
            }
        }

        panic!("impossible.");
    }

    pub fn get_items(&self) -> itertools::Product<itertools::Product<std::vec::IntoIter<Item>, std::vec::IntoIter<Option<Item>>>, Combinations<std::vec::IntoIter<Option<Item>>>> {
        let shop = Shop::new();

        let iter_weapons = shop.weapons.into_iter();
        let iter_armor = vec![shop.armor.into_iter().map(|item| Some(item)).collect(), vec![None]].concat().into_iter();
        let iter_rings = vec![shop.rings.into_iter().map(|item| Some(item)).collect(), vec![None, None]].concat().into_iter().combinations(2);

        return iter_weapons.cartesian_product(iter_armor).cartesian_product(iter_rings);
    }

    pub fn find_best(&mut self) -> i32 {
        let mut least_gold = i32::MAX;

        for ((weapon, armor), rings) in self.get_items() {
            let mut player = Player::new();
            player.buy(&Some(weapon));
            player.buy(&armor);

            for ring in rings {
                player.buy(&ring);
            }

            if self.fight(&player) {
                let gold = player.get_gold_spent();
                if gold < least_gold {
                    least_gold = gold;
                }
            }

        }

        return least_gold;
    }

    pub fn find_worst(&mut self) -> i32 {
        let mut most_gold = 0;

        for ((weapon, armor), rings) in self.get_items() {
            let mut player = Player::new();
            player.buy(&Some(weapon));
            player.buy(&armor);

            for ring in rings {
                player.buy(&ring);
            }

            if !self.fight(&player) {
                let gold = player.get_gold_spent();
                if gold > most_gold {
                    most_gold = gold;
                }
            }
        }

        return most_gold;
    }
}

impl LineMatch for Game {
    fn get_regex(&self) -> Regex {
        Regex::new(r"(?P<stat>.+?): (?P<value>\d+)").unwrap()
    }

    fn add_match(&mut self, cap: Captures) {
        let stat = String::from(cap.name("stat").unwrap().as_str());
        let value = cap.name("value").unwrap().as_str().parse::<i32>().unwrap();

        match stat.as_str() {
            "Hit Points" => self.boss.hp = value,
            "Damage" => self.boss.damage = value,
            "Armor" => self.boss.armor = value,
            _ => panic!("Unknown stat: {}", stat),
        };
    }
}

impl New<Day21> for Day21 {
    fn new(file_name: &str) -> Day21 {
        Day21 { input: Day21::get_content(file_name)}
    }
}

impl Parts for Day21 {
    fn part1(&self) -> i32 {
        let mut game = Game::new();
        for line in &self.input {
            game.add_line(line);
        }

        return game.find_best();
    }

    fn part2(&self) -> i32 {
        let mut game = Game::new();
        for line in &self.input {
            game.add_line(line);
        }

        return game.find_worst();
    }
}