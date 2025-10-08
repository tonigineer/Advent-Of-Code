use aoc::year2024::day12::*;

const EXAMPLE_PART1: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

const EXAMPLE_PART2: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE_PART1);
    assert_eq!(part1(input), 1930);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE_PART1);
    assert_eq!(part2(input), 1206);
    let input = parse(EXAMPLE_PART2);
    assert_eq!(part2(input), 368);
}
