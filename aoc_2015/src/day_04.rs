use common::{Answer, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "The Ideal Stocking Stuffer"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, 5).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, 6).into();
    }
}

fn solve(input: &str, num_zeros: usize) -> u32 {
    let mut index = 0;

    loop {
        let digest = md5::compute(format!("{input}{index}"));

        if &format!("{:?}", digest)[0..num_zeros] == format!("{:0num_zeros$}", 0) {
            return index;
        }

        index += 1
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        abcdef
    "};

    #[test]
    fn example() {
        assert_eq!(solve(SAMPLE, 5), 521640);
    }
}