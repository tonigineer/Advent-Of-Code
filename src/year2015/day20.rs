//! Infinite Elves and Infinite Houses
//!
//! Summary:

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    solve(input, true)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> usize {
    let presents: usize = input.parse().unwrap();

    // Memorize intensive solution
    let mut houses: Vec<usize> = vec![0; presents / 10];

    for elf in 1..houses.len() {
        for id in (elf..houses.len()).step_by(elf) {
            if part2 {
                if id / elf == 51 {
                    break;
                }
                houses[id] += elf * 11;
            } else {
                houses[id] += elf * 10;
            }
        }
    }

    houses.iter().position(|h| h >= &presents).unwrap()
}
