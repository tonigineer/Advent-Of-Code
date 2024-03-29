use common::{Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "I Was Told There Would Be No Math"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn new(dims: Vec<u32>) -> Self {
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];

        Present { l, w, h }
    }

    fn paper_needed(&self) -> u32 {
        return 3 * self.l * self.w + 2 * self.l * self.h + 2 * self.w * self.h;
    }

    fn ribbon_needed(&self) -> u32 {
        return 2 * self.l + 2 * self.w + self.l * self.h * self.w;
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut paper: u32 = 0;
    let mut ribbon: u32 = 0;

    for line in input.lines() {
        let mut dims: Vec<u32> = line
            .split('x')
            .map(|item| item.parse().expect("Parsing not possible"))
            .collect();
        dims.sort();

        let p = Present::new(dims);
        paper += p.paper_needed();
        ribbon += p.ribbon_needed();
    }

    if part2 {
        return ribbon;
    }
    return paper;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        2x3x4
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 58);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 34);
    }
}