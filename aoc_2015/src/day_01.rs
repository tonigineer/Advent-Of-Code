use common::{Answer, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Not Quite Lisp"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve_part1(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve_part2(input).into();
    }
}

fn solve_part1(input: &str) -> usize {
    return input.chars().count() - 2 * input.matches(')').count();
}

fn solve_part2(input: &str) -> usize {
    let mut level = 0;
    let mut result: usize = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => panic!("Char not defined."),
        }
        result = i;
        if level < 0 {
            break;
        }
    }
    return result + 1;
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1("))((((("), 3);
        assert_eq!(solve_part1("))((((("), 3);
        assert_eq!(solve_part1("(())"), 0);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(")"), 1);
        assert_eq!(solve_part2("()())"), 5);
    }
}