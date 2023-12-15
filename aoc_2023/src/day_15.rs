use common::{Answer, Solution};

use itertools::Itertools;
use std::collections::HashMap;

pub struct Day15;

impl Solution for Day15 {
    fn name(&self) -> &'static str {
        "Lens Library"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn hash(input: &str) -> u32 {
    let mut value = 0;
    for c in input.chars() {
        value = ((value + c as u32) * 17) % 256;
    }
    value
}

fn solve(input: &str, part2: bool) -> usize {
    if ! part2 {
        return input.split(',')
            .map(|s| hash(s) as usize)
            .sum::<usize>();
    }

    let mut boxes: [Vec<&str>; 256] = vec![Vec::new(); 256].try_into().expect("static");
    let mut focal_lengths: HashMap<&str, u32> = HashMap::new();

    for item in input.split(',') {
        if item.contains('-') {
            let label = &item[..&item.len()-1];
            let index = hash(label) as usize;
            let content = boxes.get(index).unwrap();

            boxes[index] = content.iter().filter(|i| i != &&label).map(|i| *i).collect_vec();
        } else {
            let (label, value) = item.split_once("=").unwrap();
            let index = hash(label) as usize;

            let content = boxes.get(index).unwrap();
            focal_lengths.insert(label, value.parse::<u32>().unwrap());

            if !content.contains(&label) {
                boxes[index].push(label);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(b_idx, bocks)| {
            bocks
                .iter()
                .enumerate()
                .map(|(l_idx, label)|
                    (b_idx+1) * (l_idx+1) * *focal_lengths.get(label).unwrap() as usize
                ).sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 1320);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 145);
    }
}