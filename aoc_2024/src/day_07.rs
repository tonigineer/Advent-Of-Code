use common::{Answer, Solution};

pub struct Day07;

impl Solution for Day07 {
    fn name(&self) -> &'static str {
        "Bridge Repair"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> u64 {
    let mut ans = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();

        let target = left.parse::<u64>().unwrap();
        let nums: Vec<u64> = right
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect();

        if are_equal(target, &nums, part2) {
            ans += target;
        }
    }

    ans
}

fn are_equal(target: u64, nums: &Vec<u64>, part2: bool) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }

    let mut nums_new = nums.clone();
    let last_elem = nums_new.pop().unwrap();

    if (target % last_elem) == 0 && are_equal(target / last_elem, &nums_new, part2) {
        return true;
    }
    if target > last_elem && are_equal(target - last_elem, &nums_new, part2) {
        return true;
    }

    let mut target_str = target.to_string();
    let elem_str = last_elem.to_string();

    if part2 && target_str.len() > elem_str.len() {
        if target_str.ends_with(&elem_str) {
            target_str.truncate(target_str.len() - elem_str.len());

            if are_equal(target_str.parse::<u64>().unwrap(), &nums_new, part2) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE: &str = indoc::indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 3749);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 11387);
    }
}
