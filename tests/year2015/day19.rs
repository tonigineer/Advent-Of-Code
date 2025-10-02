use aoc::year2015::day19::*;

const EXAMPLE: &str = "\
H => HO
H => OH
O => HH

HOH";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 4);
}

#[test]
fn part2_test() {
    // There were no examples given.
}
