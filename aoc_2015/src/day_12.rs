use common::{Answer, Solution};

use serde_json::Value;

pub struct Day12;

impl Solution for Day12 {
    fn name(&self) -> &'static str {
        "JSAbacusFramework.io"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(serde_json::from_str(input).unwrap(), false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(serde_json::from_str(input).unwrap(), true).into();
    }
}

fn solve(json: Value, part2: bool) -> i64 {
    if json.is_number() {
        return json.as_i64().unwrap();
    }

    if json.is_string() {
        return 0;
    }

    if json.is_array() {
        let mut sum_array = 0;
        for v in json.as_array().unwrap() {
            sum_array += solve(v.clone(), part2);
        }
        return sum_array;
    }

    if json.is_object() {
        let mut sum_object = 0;
        for (_, v) in json.as_object().unwrap() {
            if part2 {
                if v == "red" {
                    return 0;
                }
            }
            sum_object += solve(v.clone(), part2);
        }
        return sum_object;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        {\"a\":2,\"b\":4}
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(serde_json::from_str(SAMPLE).unwrap(), false), 6);
    }

    const SAMPLE2: &str = indoc! {"
        [1,{\"c\":\"red\",\"b\":2},3]
    "};


    #[test]
    fn example_part2() {
        assert_eq!(solve(serde_json::from_str(SAMPLE2).unwrap(), true), 4);
    }
}