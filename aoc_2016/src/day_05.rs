use common::{Answer, Solution};
use itertools::Itertools;

use md5;

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "Cube Conundrum"
    }

    fn part1(&self, input: &str) -> Answer {
        return part1(&input.parse::<String>().unwrap()).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return part2(&input.parse::<String>().unwrap()).into();
    }
}



fn part1(id: &String) -> String {
    let mut hash = String::new();

    for i in 0.. {
        let digest = md5::compute(format!("{id}{i}"));

        // Indexing returns one byte, therefore two chars, e.g., 0xFF
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            hash.push(
                format!("{:x}", digest[2] & 0x0f)
                    .chars().nth(0).unwrap()
            );
            if hash.len() == 8 {
                break;
            }
        }
    }

    return hash;
}

fn part2(id: &String) -> String {
    let mut pw: [char; 8] = ['_', '_', '_', '_', '_', '_', '_', '_'];

    for i in 0.. {
        let digest = md5::compute(format!("{id}{i}"));

        // Indexing returns one byte, therefore two chars, e.g., 0xFF
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            let pos = format!("{:x}", digest[2] & 0x0f)
                .chars().nth(0).unwrap();
            let c = format!("{:x}", digest[3] & 0xf0)
                .chars().nth(0).unwrap();
            let pos = pos.to_string().parse::<usize>().unwrap_or(8);

            if pos < 8 {
                if pw[pos] == '_' { pw[pos] = c }
            }

            if !pw.contains(&'_') { break }
        }
    }

    return pw.iter().join("");
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::{part1, part2};

    const SAMPLE: &str = indoc! {"abc"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&SAMPLE.to_string()), "18f47a30");
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&SAMPLE.to_string()), "05ace8e3");
    }
}