//! RPG Simulator 20XX
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

use itertools::iproduct;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(_input: &str) -> u32 {
    solve(false)
}

pub fn part2(_input: &str) -> u32 {
    solve(true)
}

#[derive(Clone)]
struct Character {
    hp: i32,
    damage: i32,
    armor: i32,
    costs: u32,
}

impl Character {
    fn new(weapon: &Item, armor: &Item, left_hand: &Item, right_hand: &Item) -> Self {
        Self {
            hp: 100,
            damage: weapon.damage + left_hand.damage + right_hand.damage,
            armor: armor.armor + left_hand.armor + right_hand.armor,
            costs: weapon.costs + armor.costs + left_hand.costs + right_hand.costs,
        }
    }

    fn battle(&self, opponent: &Character) -> bool {
        let mut hp_self = self.hp;
        let mut hp_opp = opponent.hp;
        loop {
            hp_opp -= std::cmp::max(self.damage - opponent.armor, 1);
            hp_self -= std::cmp::max(opponent.damage - self.armor, 1);
            if hp_opp <= 0 {
                return true;
            };
            if hp_self <= 0 {
                return false;
            };
        }
    }
}

struct Item {
    costs: u32,
    damage: i32,
    armor: i32,
}

fn solve(part2: bool) -> u32 {
    let weapons: Vec<Item> = vec![
        Item { costs: 8, damage: 4, armor: 0 },
        Item { costs: 10, damage: 5, armor: 0 },
        Item { costs: 25, damage: 6, armor: 0 },
        Item { costs: 40, damage: 7, armor: 0 },
        Item { costs: 74, damage: 8, armor: 0 },
    ];

    let armors: Vec<Item> = vec![
        Item { costs: 13, damage: 0, armor: 1 },
        Item { costs: 31, damage: 0, armor: 2 },
        Item { costs: 53, damage: 0, armor: 3 },
        Item { costs: 75, damage: 0, armor: 4 },
        Item { costs: 102, damage: 0, armor: 5 },
        Item { costs: 0, damage: 0, armor: 0 },
    ];

    let rings: Vec<Item> = vec![
        Item { costs: 25, damage: 1, armor: 0 },
        Item { costs: 50, damage: 2, armor: 0 },
        Item { costs: 100, damage: 3, armor: 0 },
        Item { costs: 20, damage: 0, armor: 1 },
        Item { costs: 40, damage: 0, armor: 2 },
        Item { costs: 80, damage: 0, armor: 3 },
        Item { costs: 0, damage: 0, armor: 0 },
    ];

    let boss = Character { hp: 109, damage: 8, armor: 2, costs: 0 };

    if part2 {
        return iproduct!(&weapons, &armors, &rings, &rings)
            .filter(|(_, _, r1, r2)| r1.costs != r2.costs)
            .map(|(w, a, r1, r2)| Character::new(w, a, r1, r2))
            .filter(|h| !h.battle(&boss))
            .max_by(|a, b| a.costs.cmp(&b.costs))
            .unwrap()
            .costs;
    }

    iproduct!(&weapons, &armors, &rings, &rings)
        .filter(|(_, _, r1, r2)| r1.costs != r2.costs)
        .map(|(w, a, r1, r2)| Character::new(w, a, r1, r2))
        .filter(|h| h.battle(&boss))
        .min_by(|a, b| a.costs.cmp(&b.costs))
        .unwrap()
        .costs
}
