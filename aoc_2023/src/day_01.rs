use common::{Answer, Solution};

pub struct Day01;


impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Trebuchet?!"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}


fn solve(input: &str, part2: bool) -> u32 {
    let mut input = input.to_string();
    if part2 {
        input = input
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "4")
            .replace("five", "5e")
            .replace("six", "6")
            .replace("seven", "7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
    };

    let mut result = 0;

    for line in input.lines() {
        let mut numbers = "".to_string();

        for char in line.chars() {
            if char.is_ascii_digit() {
                numbers = format!("{}{}", numbers.chars().nth(0).unwrap_or(char), char);
            }
        }

        if numbers.len() == 1 { numbers = format!("{}{}", numbers, numbers) }
        result += numbers.parse::<u32>().unwrap();
    }
    return result;
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 142);
    }

    const SAMPLE2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};
 
    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE2, true), 281);
    }
}