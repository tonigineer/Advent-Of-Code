use common::{Answer, Solution};
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Mull It Over"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut enable_table: Vec<bool> = vec![true; input.len()];
    let re_do = Regex::new(r"do\(\)|don\'t\(\)").unwrap();
 
    for cap in re_do.captures_iter(input) {
        if let Some(matched) = cap.get(0) {
            if matched.as_str() == "do()" {
                enable_table[matched.start()..].fill(true);
            }
            if matched.as_str() == "don't()" {
                enable_table[matched.start()..].fill(false);
            }
        }
    }
    
    let mut ans = 0;
    let re_mul = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

    for cap in re_mul.captures_iter(input) {
        if part2 {
            if let Some(matched) = cap.get(0) {
                if enable_table[matched.start()] == false {continue}
            }
        }
        
        let x1 = &cap[1].parse::<u32>().unwrap();
        let x2 = &cap[2].parse::<u32>().unwrap();
        ans += x1 * x2;
    }
    
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE_1: &str = indoc::indoc! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};

    const SAMPLE_2: &str = indoc::indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE_1, false), 161);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE_2, true), 48);
    }
}
