use crate::days::day::{New, Parts};
use crate::days::utils::LineMatch;
use regex::{Regex, Captures};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

#[derive(PartialEq,Copy,Clone)]
enum Effect { MISSILE, DRAIN, SHIELD, POISON, RECHARGE }

pub struct Day22 {
    input: Vec<String>,
}

struct Player {
    mana: i32,
    mana_spent: i32,
    hp: i32,
    armor: i32,
}

impl Player {
    pub fn new() -> Self {
        Player { mana: 500, mana_spent:0, hp: 50, armor: 0}
    }
}

#[derive(Copy,Clone)]
struct Boss {
    pub hp: i32,
    pub damage: i32,
}

impl Boss {
    pub fn new() -> Self {
        Boss { hp: 0, damage: 0}
    }
}

struct Spell {
    cost: i32,
    duration: i32,
    effect: Effect,
}

impl Spell {
    pub fn new(effect: Effect) -> Self {
        match effect {
            Effect::MISSILE => Spell { cost: 53, duration: 0, effect: Effect::MISSILE },
            Effect::DRAIN => Spell { cost: 73, duration: 0, effect: Effect::DRAIN },
            Effect::SHIELD => Spell { cost: 113, duration: 6, effect: Effect::SHIELD },
            Effect::POISON => Spell { cost: 173, duration: 6, effect: Effect::POISON },
            Effect::RECHARGE => Spell { cost: 229, duration: 6, effect: Effect::RECHARGE },
        }
    }

    pub fn can_be_cast(&self, player: &Player, other_spells: &Vec<Spell>) -> bool {
        if player.mana < self.cost {
            // println!("no mana for '{}' ({} / {})", self.name, player.mana, self.cost);
            return false;
        }

        for spell in other_spells {
            // note: spell can be cast again if the previous spell ends this turn
            if spell.effect == self.effect && spell.duration > 1 {
                return false;
            }
        }

        return true;
    }

    pub fn cast(&self, player: &mut Player, boss: &mut Boss) {
        player.mana -= self.cost;
        player.mana_spent += self.cost;

        match self.effect {
            Effect::MISSILE => boss.hp -= 4,
            Effect::SHIELD => player.armor += 7,
            Effect::DRAIN => {
                boss.hp -= 2;
                player.hp += 2;
            },
            _ => {},
        }
    }

    pub fn run(&mut self, player: &mut Player, boss: &mut Boss) {
        if self.duration == 0 {
            return;
        }
        self.duration -= 1;

        match self.effect {
            Effect::POISON => {
                boss.hp -= 3;
            },
            Effect::RECHARGE => {
                player.mana += 101;
            },
            _ => {}
        }

        if self.duration == 0 {
            if self.effect == Effect::SHIELD {
                player.armor -= 7;
            }
            return;
        }
    }
}

impl Clone for Spell {
    fn clone(&self) -> Self {
        Spell::new(self.effect)
    }
}

struct Game {
    boss: Boss,
}

impl Game {
    pub fn new() -> Game {
        Game { boss: Boss::new() }
    }

    pub fn fight(player: &mut Player, boss: &mut Boss, spells: &Vec<Spell>, hard_mode: &bool) -> (bool, bool) {
        let mut active_spells: Vec<Spell> = Vec::new();

        for round in 0..usize::MAX {
            // ======> player turn <======
            if *hard_mode {
                player.hp -= 1;
                if player.hp <= 0 {
                    return (false, false);
                }
            }

            if round >= spells.len() {
                // kept me alive so far...will try again with this at start
                return (false, true);
            }

            if !spells[round].can_be_cast(player, &active_spells) {
                return (false, false);
            }

            for spell in &mut active_spells {
                spell.run(player, boss);
            }

            let new_spell = spells[round].clone();
            new_spell.cast(player, boss);
            active_spells.push(new_spell);

            // ======> boss turn <======
            for spell in &mut active_spells {
                spell.run(player, boss);
            }

            if boss.hp <= 0 {
                // println!("fight ends at {} (WIN! @ {} mana)", round, player.mana_spent);
                return (true, false);
            }

            player.hp -= vec![boss.damage - player.armor, 1].into_iter().max().unwrap();
            if player.hp <= 0 {
                return (false, false);
            }
        }

        panic!("impossible.");
    }

    /*
    pub fn try_list(&self, spell_list: &Vec<Spell>, permutations: usize) -> i32 {
        println!("try {} permutations", permutations);

        let mut least_mana = i32::MAX;
        for spells in spell_list.clone().into_iter().permutations(permutations) {
            let mut boss = self.boss.clone();
            let mut player = Player::new();
            let fight_spells = spells.clone();

            let result = Self::fight(&mut player, &mut boss, &fight_spells);
            if result && player.mana_spent < least_mana {
                least_mana = player.mana_spent;
            }
        }
        return least_mana;
    }
    */

    pub fn try_list_threaded(&self, spell_lists: &Vec<Vec<Spell>>, add_spells: &Vec<Spell>, hard_mode: &bool) -> (i32, Vec<Vec<Spell>>) {
        let pool = ThreadPool::new(8);
        let (tx, rx) = channel();

        let mut num_fights = 0;
        for spell_list in spell_lists {
            for spell in add_spells {
                num_fights += 1;

                // add spell to list
                let mut fight_spells = spell_list.clone();
                fight_spells.push(spell.clone());

                // run fight
                let tx = tx.clone();
                let mut boss = self.boss.clone();
                let mut player = Player::new();
                let hard_mode = hard_mode.clone();

                pool.execute(move || {
                    let result = Self::fight(&mut player, &mut boss, &fight_spells, &hard_mode);

                    match tx.send(((result.0, match result.1 {
                        true => Some(fight_spells),
                        false => None,
                    }), player.mana_spent)) { _ => {}};
                });
            }
        }

        let mut least_mana = i32::MAX;
        let mut kept_alive = Vec::new();
        for (result, mana_spent) in rx.iter().take(num_fights as usize) {
            match result.1 {
                Some(list) => kept_alive.push(list),
                None => {},
            }
            if result.0 && mana_spent < least_mana {
                least_mana = mana_spent;
            }
        }
        return (least_mana, kept_alive);
    }

    pub fn find_best(&mut self, hard_mode: bool) -> i32 {
        let spells = vec![
            Spell::new(Effect::MISSILE),
            Spell::new(Effect::DRAIN),
            Spell::new(Effect::SHIELD),
            Spell::new(Effect::POISON),
            Spell::new(Effect::RECHARGE),
        ];

        let mut prev_list: Option<Vec<Vec<Spell>>> = None;
        let mut prev_result = None;
        for _i in 1..usize::MAX {
            let result = self.try_list_threaded(&match prev_list {
                Some(list) => list.clone(),
                None => vec![Vec::new()],
            }, &spells, &hard_mode);

            prev_list = Some(result.1);
            if result.0 < i32::MAX {
                let best = match prev_result {
                    Some(prev) => if prev < result.0 { prev } else { result.0 },
                    None => result.0,
                };

                prev_list = Some(prev_list.clone().unwrap().into_iter().filter(|list| {
                    // note: 53 is the minimum spell cost
                    return (list.into_iter().map(|item| item.cost).sum::<i32>() + 53) < best;
                }).collect());

                if prev_list.as_ref().unwrap().len() == 0 {
                    return best;
                }
                prev_result = Some(best);
            } else if prev_result != None {
                return prev_result.unwrap();
            }

        }

        panic!("impossible.");
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
            _ => panic!("Unknown stat: {}", stat),
        };
    }
}

impl New<Day22> for Day22 {
    fn new(file_name: &str) -> Day22 {
        Day22 { input: Day22::get_content(file_name)}
    }
}

impl Parts for Day22 {

    fn part1(&self) -> i32 {
        let mut game = Game::new();
        for line in &self.input {
            game.add_line(line);
        }

        return game.find_best(false);
    }

    fn part2(&self) -> i32 {
        let mut game = Game::new();
        for line in &self.input {
            game.add_line(line);
        }

        return game.find_best(true);
    }
}