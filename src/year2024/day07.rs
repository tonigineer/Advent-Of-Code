//! Bridge Repair
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u64 {
    solve(input, false)
}

pub fn part2(input: &str) -> u64 {
    solve(input, true)
}

fn solve(input: &str, part2: bool) -> u64 {
    let mut ans = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();

        let target = left.parse::<u64>().unwrap();
        let nums: Vec<u64> = right.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect();

        if are_equal(target, &nums, part2) {
            ans += target;
        }
    }

    ans
}

fn are_equal(target: u64, nums: &[u64], part2: bool) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }

    let mut nums_new = nums.to_owned();
    let last_elem = nums_new.pop().unwrap();

    if target.is_multiple_of(last_elem) && are_equal(target / last_elem, &nums_new, part2) {
        return true;
    }
    if target > last_elem && are_equal(target - last_elem, &nums_new, part2) {
        return true;
    }

    let mut target_str = target.to_string();
    let elem_str = last_elem.to_string();

    if part2 && target_str.len() > elem_str.len() && target_str.ends_with(&elem_str) {
        target_str.truncate(target_str.len() - elem_str.len());

        if are_equal(target_str.parse::<u64>().unwrap(), &nums_new, part2) {
            return true;
        }
    }

    false
}
