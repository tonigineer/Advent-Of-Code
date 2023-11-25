use std::collections::VecDeque;

enum Spells {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge
}

#[derive(Clone, Copy)]
struct Game {
    player_hp: i32,
    player_mana: i32,
    player_armor: i32,
    boss_hp: i32,
    boss_damage: i32,
    poison_timer: i32,
    shield_timer: i32,
    recharge_timer: i32,
    mana_spent: i32,
}

impl Game {
    fn play(&self, part2: bool) -> i32 {
        let mut least_mana_spent: i32 = i32::MAX;
        let mut queue = VecDeque::new();

        let game: Game = *self;
        queue.push_back((game, Spells::Missile));
        queue.push_back((game, Spells::Drain));
        queue.push_back((game, Spells::Shield));
        queue.push_back((game, Spells::Poison));
        queue.push_back((game, Spells::Recharge));

        while let Some((mut state, spell)) = queue.pop_front() {
            // Hard mode
            if part2 {
                state.player_hp -= 1;
                if state.player_hp <= 0 { continue }
            }

            state.player_armor = 0;
            if state.shield_timer > 0 {
                state.player_armor = 7;
                state.shield_timer -= 1;
            }
            if state.recharge_timer > 0 {
                state.player_mana += 101;
                state.recharge_timer -= 1;
            }
            if state.poison_timer > 0 {
                state.boss_hp -= 3;
                state.poison_timer -= 1;
            }

            if state.boss_hp <= 0 {
                least_mana_spent = least_mana_spent.min(state.mana_spent);
                continue;
            }

            match spell{
                Spells::Missile => {
                    state.boss_hp -= 4;
                    state.mana_spent += 53;
                    state.player_mana -= 53;
                },
                Spells::Drain => {
                    state.boss_hp -= 2;
                    state.player_hp += 2;
                    state.mana_spent += 73;
                    state.player_mana -= 73;
                }
                Spells::Shield => {
                    state.shield_timer = 6;
                    state.mana_spent += 113;
                    state.player_mana -= 113;
                }
                Spells::Poison => {
                    state.poison_timer = 6;
                    state.mana_spent += 173;
                    state.player_mana -= 173;
                }
                Spells::Recharge => {
                    state.recharge_timer = 5;
                    state.mana_spent += 229;
                    state.player_mana -= 229;
                }
            }

            if state.player_mana < 0 { continue }
            if state.mana_spent > least_mana_spent { continue }

            // Boss
            state.player_armor = 0;
            if state.shield_timer > 0 {
                state.player_armor = 7;
                state.shield_timer -= 1;
            }
            if state.recharge_timer > 0 {
                state.player_mana += 101;
                state.recharge_timer -= 1;
            }
            if state.poison_timer > 0 {
                state.boss_hp -= 3;
                state.poison_timer -= 1;
            }

            if state.boss_hp <= 0 {
                least_mana_spent = least_mana_spent.min(state.mana_spent);
                continue;
            }

            state.player_hp -= (state.boss_damage - state.player_armor).max(1);
            if state.player_hp <= 0 { continue }

            queue.push_back((state, Spells::Missile));
            queue.push_back((state, Spells::Drain));
            queue.push_back((state, Spells::Shield));
            queue.push_back((state, Spells::Poison));
            queue.push_back((state, Spells::Recharge));
        }
        return least_mana_spent;
    }
}

fn main() {
    // let input = include_str!("../../inputs/21.in").trim();
    let game = Game {
        player_hp: 50, player_mana: 500, player_armor: 0, boss_hp: 51, boss_damage: 9,
        poison_timer: 0, shield_timer: 0, recharge_timer: 0, mana_spent: 0
    };

    print!("󰎤 {} ", game.play(false));
    print!("󰎧 {} ", game.play(true));
}

