use common::{Answer, Solution};
use std::collections::HashSet;

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "Scratchcards"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut total = 0;
    let mut cards: [u32; 1000] = [0; 1000];  // Should be large enough :)

    for (idx, card) in input.lines().enumerate() {
        cards[idx] += 1;
        let (_, draw) = card.split_once(": ").unwrap();
        let (win, my) = draw.split_once(" | ").unwrap();

        let mut w: HashSet<u32> = HashSet::new();
        for number in win.split_whitespace().into_iter() {
            w.insert(number.parse::<u32>().unwrap());
        }

        let mut m: HashSet<u32> = HashSet::new();
        for number in my.split_whitespace().into_iter() {
            m.insert(number.parse::<u32>().unwrap());
        }

        let matches = m.len() - m.difference(&w).count();

        if part2 {
            for i in idx+1..idx+1+matches {
                cards[i] += 1 * cards[idx];
            }
        } else {
            if matches > 0 {
                total += (2 as i32).pow(matches as u32 - 1) as u32;
            }
        }
    }

    if part2 { cards.iter().sum() } else { return total }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 30);
    }
}