use common::{Answer, Solution};
use counter::Counter;

pub struct Day07;

impl Solution for Day07 {
    fn name(&self) -> &'static str {
        "Camel Cards"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut plays: Vec<(u32, u32, String)> = Vec::new();

    let hands: Vec<(&str, u32)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand, bid.parse::<u32>().unwrap())
        })
        .collect();

    for (hand, bid) in hands.iter() {
        let mut cards = hand.chars().collect::<Counter<_>>().most_common_ordered();

        // Simply put all joker on the highest other kind
        if part2 && cards.len() > 1 {
            for i in (0..cards.len()).rev() {
                if cards[i].0 == 'J' {
                    let n = cards[i].1;
                    cards.remove(i);
                    cards[0].1 += n;
                    break;
                }
            }
        }

        let value = match cards
            .iter_mut()
            .fold("".to_string(), | acc, (_, n) | format!("{acc}{n}")).as_str()
        {
            "5" => 6,
            "41" => 5,
            "32" => 4,
            "311" => 3,
            "221" => 2,
            "2111" => 1,
            "11111" => 0,
            _ => panic!("Game not implemented")
        };

        plays.push(
            (*bid, value, hand.to_string()
                .replace("A", "E")
                .replace("K", "D")
                .replace("Q", "C")
                .replace("J", if part2 {"."} else {"B"})
                .replace("T", "A")
            )
        );
    }

    plays.sort_by(|a, b| b.1.cmp(&a.1).reverse().then(b.2.cmp(&a.2).reverse()));

    return plays
        .iter()
        .enumerate()
        .fold(0, | acc, (i, play) | acc + play.0 * (i+1) as u32)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 6440);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 5905);
    }
}

