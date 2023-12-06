use common::{Answer, Solution};

use std::cmp::min;

pub struct Day14;

impl Solution for Day14 {
    fn name(&self) -> &'static str {
        "Reindeer Olympics"
    }

    fn part1(&self, input: &str) -> Answer {
        let game: Game = input.into();
        return game.play(2503).into();
    }

    fn part2(&self, input: &str) -> Answer {
        let mut game: Game = input.into();
        return game.play_for_points(2503).into();
    }
}

#[derive(Debug)]
struct Deer {
    _name: String,
    speed: u32,
    fly_time: u32,
    pause_time: u32,
    points: u32,
}

impl From<&str> for Deer {
    fn from(s: &str) -> Deer {
        let token = s.trim_end_matches(".").split(" ").collect::<Vec<&str>>();
        return Deer {
            _name: token[0].to_string(),
            speed: token[3].parse().unwrap(),
            fly_time: token[6].parse().unwrap(),
            pause_time: token[13].parse().unwrap(),
            points: 0,
        };
    }
}

impl Deer {
    fn get_distance(&self, t: u32) -> u32 {
        let cycle_time = self.fly_time + self.pause_time;
        let num_full_cycle = t / cycle_time;
        let t_remain = t % cycle_time;
        return num_full_cycle * self.fly_time * self.speed
            + min(t_remain, self.fly_time) * self.speed;
    }
}

#[derive(Debug)]
struct Game {
    deer: Vec<Deer>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Game {
        let mut deer: Vec<Deer> = Vec::new();
        for line in s.lines() {
            let d: Deer = line.into();
            deer.push(d);
        }
        return Game { deer: deer };
    }
}

impl Game {
    fn play(&self, target_time: u32) -> u32 {
        return self
            .deer
            .iter()
            .max_by_key(|d| d.get_distance(target_time))
            .unwrap()
            .get_distance(target_time); // one extra calc, but should be fine :)
    }

    fn play_for_points(&mut self, target_time: u32) -> u32 {
        for i in 1..=target_time {
            let max_distance = self.play(i);
            self.deer
                .iter_mut()
                .filter(|d| d.get_distance(i) == max_distance)
                .for_each(|d| d.points += 1)
        }
        // NOTE: this solution is really bad, because distance are calculated twice

        return self.deer.iter().max_by_key(|d| d.points).unwrap().points;
    }
}
