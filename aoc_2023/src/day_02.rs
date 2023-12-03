use common::{Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Cube Conundrum"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}


fn solve(input: &str, part2: bool) -> u32 {
    let mut result1 = 0;
    let mut result2 = 0;

    for (idx, line) in input.lines().enumerate() {
        let (_, games) = line.split_once(": ").unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let mut valid = true;
        for game in games.split("; ") {
            for draw in game.split(", ") {
                let (num_str, color) = draw.split_once(" ").unwrap();
                let num = num_str.parse::<u32>().unwrap();
                
                if part2 {
                    match color {
                        "red" => red = red.max(num),
                        "green" => green = green.max(num),
                        "blue" => blue = blue.max(num),
                        _ => unimplemented!()
                    };
                } else {
                    match color {
                        "red" => { if num > 12 { valid = false } },
                        "green" => { if num > 13 { valid = false } },
                        "blue" => { if num > 14 { valid = false } },
                        _ => unimplemented!()
                    };
                }
            }
        };
        if valid { result1 += idx as u32 + 1 }
        result2 += red * green * blue;
    }
    if part2 { return result2 } else { return result1 };
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 8);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 2286);
    }
}