use crate::Answer;

pub trait Solution {
    fn name(&self) -> &'static str;

    fn part1(&self, input: &str) -> Answer;

    fn part2(&self, input: &str) -> Answer;
}
