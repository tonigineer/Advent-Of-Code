use common::{Answer, Solution};

use std::collections::{ HashMap, HashSet };

pub struct Day19;

impl Solution for Day19 {
    fn name(&self) -> &'static str {
        "Medicine for Rudolph"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve2(input).into();
    }
}


fn solve(input: &str) -> usize {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let mut distinct_molecules:HashSet<String> = HashSet::new();

    for line in replacements.lines() {
        let (pattern, substitute) = line.split_once(" => ").unwrap();

        for i in 0..=molecule.len()-pattern.len() {
            if molecule[i..i+pattern.len()].to_string() == pattern {
                distinct_molecules.insert(
                    format!("{}{}{}",
                        molecule[0..i].to_string(),
                        substitute,
                        molecule[i+pattern.len()..molecule.len()].to_string()
                    )
                );
            }
        }
    }
    return distinct_molecules.len();
}

fn solve2(input: &str) -> usize {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let mut rm :HashMap<&str, &str> = HashMap::new();

    for line in replacements.lines() {
        let (pattern, substitute) = line.split_once(" => ").unwrap();
        rm.insert(substitute, pattern);
    };

    let mut sorted_keys: Vec<&&str> = rm.keys()
        .into_iter()
        .collect::<Vec<_>>();
    sorted_keys.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut current_molecule: String = molecule.to_string();
    let mut count = 0;
    loop {
        for substitute in sorted_keys.iter_mut() {
            if current_molecule.contains(*substitute) {
                current_molecule = current_molecule.replacen(*substitute, rm.get(*substitute).unwrap(), 1);
                count += 1;
                break;
            }
        }

        if current_molecule == "e".to_string() { break }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        H => HO
        H => OH
        O => HH

        HOH
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 4);
    }
}