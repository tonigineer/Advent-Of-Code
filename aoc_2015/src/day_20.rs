use common::{Answer, Solution};

pub struct Day20;

impl Solution for Day20 {
    fn name(&self) -> &'static str {
        "Infinite Elves and Infinite Houses"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}


fn solve(input: &str, part2: bool) -> usize {
    let presents: usize = input.parse().unwrap();

    // Memorize intensive solution
    let mut houses: Vec<usize> = vec![0; presents/10];

    for elf in 1..houses.len(){
        for id in (elf..houses.len()).step_by(elf) {
            if part2 {
                if id / elf == 51 { break }
                houses[id] += elf * 11;
            } else { 
                houses[id] += elf * 10;
            }
        }
    }

    return houses.iter().position(|h| h >= &presents).unwrap();
}
