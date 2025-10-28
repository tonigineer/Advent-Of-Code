//! Aunt Sue
//!
//! Summary: Splitting all aunts and checking things in hashmap.

use hashbrown::HashMap;

type InputParsed<'a> = (&'a str, HashMap<&'a str, u8>);

pub fn parse(input: &'_ str) -> InputParsed<'_> {
    let memories: HashMap<&'static str, u8> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    (input.trim(), memories)
}

pub fn part1(input: &InputParsed) -> usize {
    let (aunts, memories) = input;

    for (idx, line) in aunts.trim().lines().enumerate() {
        let (_, things) = line.split_once(":").unwrap();

        let all_match = things.split(',').all(|pair| {
            let (k, v) = pair.split_once(":").unwrap();
            let amount: u8 = v.trim().parse().unwrap();
            &amount == memories.get(k.trim()).unwrap()
        });

        if all_match {
            return idx + 1;
        }
    }

    unreachable!()
}

pub fn part2(input: &InputParsed) -> usize {
    let (aunts, memories) = input;

    for (idx, line) in aunts.trim().lines().enumerate() {
        let (_, things) = line.split_once(":").unwrap();

        let all_match = things.split(',').all(|pair| {
            let (k, v) = pair.split_once(":").unwrap();
            let k = k.trim();
            let amount: u8 = v.trim().parse().unwrap();
            match k {
                "cats" | "trees" => &amount > memories.get(k.trim()).unwrap(),
                "pomeranians" | "goldfish" => &amount < memories.get(k.trim()).unwrap(),
                _ => &amount == memories.get(k.trim()).unwrap(),
            }
        });

        if all_match {
            return idx + 1;
        }
    }

    unreachable!()
}
