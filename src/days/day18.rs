use crate::days::day::{New, Parts};
use itertools::Itertools;

pub struct Day18 {
    input: Vec<String>,
}

pub struct GameOfLights {
    lights: Vec<Vec<bool>>,
    len: usize,
}

impl GameOfLights {
    pub fn new() -> Self {
        GameOfLights { lights: Vec::new(), len: 0 }
    }

    pub fn init(&mut self, input: &Vec<String>) {
        self.len = input.len();

        let mut rows = Vec::with_capacity(self.len);
        for i in 0..self.len {
            let line_data: Vec<&str> = input[i].split("").collect();
            let mut columns = Vec::with_capacity(self.len);
            for j in 1..self.len + 1 {
                columns.push(line_data[j] == "#");
            }

            rows.push(columns);
        }
        self.lights = rows;
    }

    pub fn fix_corners(&mut self) {
        self.lights[0][0] = true;
        self.lights[0][self.len - 1] = true;
        self.lights[self.len - 1][0] = true;
        self.lights[self.len - 1][self.len - 1] = true;
    }

    pub fn next(&mut self) {
        let mut new_ligts = self.lights.clone();

        for (x, y) in (0..self.len).cartesian_product(0..self.len) {
            let count = self.count_neighbors(&(x as i32), &(y as i32));
            let (x, y) = (x as usize, y as usize);

            if self.lights[x][y] && !(count == 2 || count == 3) {
                new_ligts[x][y] = false;
            } else if !self.lights[x][y] && count == 3 {
                new_ligts[x][y] = true;
            }
        }

        self.lights = new_ligts;
    }

    pub fn count(&self) -> i32 {
        let mut count = 0;
        for (x, y) in (0..self.len).cartesian_product(0..self.len) {
            count += if self.lights[x][y] { 1 } else { 0 };
        }
        return count;
    }

    fn count_neighbors (&self, i: &i32, j: &i32) -> i32 {
        let mut count = 0;
        let iter = (i - 1..i + 2).cartesian_product(j - 1..j + 2).filter(|(x, y)| !(x == i && y == j) && *x > -1 && *x < self.len as i32 && *y > -1 && *y < self.len as i32);
        for (x, y) in iter {
            if self.lights[x as usize][y as usize] {
                count += 1;
            }
        }
        return count;
    }
}

impl New<Day18> for Day18 {
    fn new(file_name: &str) -> Day18 {
        Day18 { input: Day18::get_content(file_name)}
    }
}

impl Parts for Day18 {
    fn part1(&self) -> i32 {
        let mut game = GameOfLights::new();
        game.init(&self.input);
        for _i in 0..100 {
            game.next();
        }
        return game.count();
    }

    fn part2(&self) -> i32 {
        let mut game = GameOfLights::new();
        game.init(&self.input);
        for _i in 0..100 {
            game.fix_corners();
            game.next();
        }

        game.fix_corners();
        return game.count();
    }
}