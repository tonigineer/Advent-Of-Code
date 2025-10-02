use aoc::year2024::day22::*;

const EXAMPLE_PART1: &str = "\
1
10
100
2024";

const EXAMPLE_PART2: &str = "\
1
2
3
2024";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE_PART1);
    assert_eq!(part1(&input), 37327623);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE_PART2);
    assert_eq!(part2(&input), 23);
}
