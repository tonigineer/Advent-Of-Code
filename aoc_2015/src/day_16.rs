use common::{Answer, Solution};

use std::collections::HashMap;

pub struct Day16;

impl Solution for Day16 {
    fn name(&self) -> &'static str {
        "Aunt Sue"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> usize {
    let mut things: HashMap<String, u32> = HashMap::new();

    things.insert("children".to_string(), 3 );
    things.insert("cats".to_string(), 7 );
    things.insert("samoyeds".to_string(), 2 );
    things.insert("pomeranians".to_string(), 3 );
    things.insert("akitas".to_string(), 0 );
    things.insert("vizslas".to_string(), 0 );
    things.insert("goldfish".to_string(), 5 );
    things.insert("trees".to_string(), 3 );
    things.insert("cars".to_string(), 2 );
    things.insert("perfumes".to_string(), 1 );

    for (idx, line) in input.lines().enumerate() {
        let modded_line = line.replace(",","").replace(":","");
        let token = modded_line.split(" ").collect::<Vec<&str>>();
        let mut found = true;

        for i in (2..=7).step_by(2) {
            let target = things.get(token[i]).unwrap();
            let actual = token[i+1].parse::<u32>().unwrap();

            // More nesting! :D
            if part2 {
                if ["cats", "trees"].contains(&token[i]) {
                    if *target >= actual {
                        found = false;
                    }
                } else if ["pomeranians", "goldfish"].contains(&token[i]) {
                    if *target <= actual {
                        found = false;
                    }
                } else {
                    if *target != actual {
                        found = false;
                    }
                }
            } else {
                if *target != actual {
                    found = false;
                }
            }
        }
        if found { return idx+1 }
    }
    panic!("No solution found, something's fishy.")
}
