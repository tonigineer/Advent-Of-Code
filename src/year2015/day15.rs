//! Science for Hungry People
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> i64 {
    let mut dish: Dish = input.into();
    dish.highest_score(100, false)
}

pub fn part2(input: &str) -> i64 {
    let mut dish: Dish = input.into();
    dish.highest_score(100, true)
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl From<&str> for Ingredient {
    fn from(s: &str) -> Ingredient {
        let mod_s = s.replace(":", "").replace(",", "");
        let token = mod_s.split(" ").collect::<Vec<_>>();
        return Ingredient {
            name: token[0].to_string(),
            capacity: token[2].parse().unwrap(),
            durability: token[4].parse().unwrap(),
            flavor: token[6].parse().unwrap(),
            texture: token[8].parse().unwrap(),
            calories: token[10].parse().unwrap(),
        };
    }
}

#[derive(Debug)]
struct Dish {
    ingredients: HashMap<String, Ingredient>,
    capacity_total: i64,
    durability_total: i64,
    flavor_total: i64,
    texture_total: i64,
    calories_total: i64,
}

impl From<&str> for Dish {
    fn from(s: &str) -> Dish {
        let mut ingredients: HashMap<String, Ingredient> = HashMap::new();
        for line in s.lines() {
            let i: Ingredient = line.into();
            ingredients.insert(i.name.to_string(), i);
        }
        return Dish {
            ingredients: ingredients,
            capacity_total: 0,
            durability_total: 0,
            flavor_total: 0,
            texture_total: 0,
            calories_total: 0,
        };
    }
}

impl Dish {
    fn highest_score(&mut self, num_ingredients: usize, part2: bool) -> i64 {
        let mut highest_score: i64 = 0;
        for com in self.ingredients.keys().combinations_with_replacement(num_ingredients) {
            for (amount, ingredient_name) in com.iter().dedup_with_count() {
                self.capacity_total +=
                    self.ingredients.get(&ingredient_name.to_string()).unwrap().capacity
                        * amount as i64;
                self.durability_total +=
                    self.ingredients.get(&ingredient_name.to_string()).unwrap().durability
                        * amount as i64;
                self.flavor_total +=
                    self.ingredients.get(&ingredient_name.to_string()).unwrap().flavor
                        * amount as i64;
                self.texture_total +=
                    self.ingredients.get(&ingredient_name.to_string()).unwrap().texture
                        * amount as i64;
                self.calories_total +=
                    self.ingredients.get(&ingredient_name.to_string()).unwrap().calories
                        * amount as i64;
            }

            let score = (max(self.capacity_total, 0)
                * max(self.durability_total, 0)
                * max(self.flavor_total, 0)
                * max(self.texture_total, 0)) as i64;

            if part2 {
                if score > highest_score && self.calories_total == 500 {
                    highest_score = score
                }
            } else {
                if score > highest_score {
                    highest_score = score
                }
            }

            self.capacity_total = 0;
            self.durability_total = 0;
            self.flavor_total = 0;
            self.texture_total = 0;
            self.calories_total = 0;
        }

        return highest_score;
    }
}
