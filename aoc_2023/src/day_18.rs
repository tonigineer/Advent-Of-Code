use common::{ Answer, Solution };

pub struct Day18;

impl Solution for Day18 {
    fn name(&self) -> &'static str {
        "Lavaduct Lagoon"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u64 {
    let mut points = Vec::<(i64, i64)>::new();
    let mut x: i64= 0;
    let mut y: i64 = 0;
    points.push((x, y));

    let mut b = 0;

    for line in input.lines() {
        let (first, second) = line.split_once(" (#").unwrap();
        let (mut dir, length) = first
            .split_once(' ').unwrap();

        let mut length = length.parse::<i64>().unwrap();

        if part2 {
            dir = match second.trim_end_matches(')').trim_start_matches('#').chars().last().unwrap() {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => panic!("Not supposed to happen, ffs.")
            };

            length = i64::from_str_radix(
                &second.trim_end_matches(')').replace('#', "0x")[0..5], 16
            ).unwrap();
        }

        (x, y) = match dir {
            "U" => (x, y - length),
            "D" => (x, y + length),
            "L" => (x - length, y),
            "R" => (x + length, y),
            _ => panic!("Not supposed to happen, ffs.")
        };

        b += length;
        points.push((x, y));
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = 0;
    for (i, p) in points.iter().enumerate() {
        let i_before = if i == 0 {points.len() - 1} else {i - 1};
        let i_after = if i + 1 >= points.len() {0} else {i+1};
        area += p.0 * (points[i_after].1 - points[i_before].1);
    }
    area /= 2;

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let i = area - b / 2 + 1;
    return (i + b) as u64;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 62);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 952408144115);
    }
}