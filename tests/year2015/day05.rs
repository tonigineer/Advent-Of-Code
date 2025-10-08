use aoc::year2015::day05::*;

const EXAMPLE_PART1: &str = "\
ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

const EXAMPLE_PART2: &str = "\
qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE_PART1);
    assert_eq!(part1(input), 2);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE_PART2);
    assert_eq!(part2(input), 2);
}
