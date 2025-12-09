use aoc::year2025::day06::*;

// Note: Whitespace at the end are necessary here for part 2! Do not auto format.
const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), 4277556);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), 3263827);
}
