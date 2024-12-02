use common::Solution;

mod day_01;
mod day_02;

#[rustfmt::skip]
pub const ALL: &[&dyn Solution] = &[
    &day_01::Day01,
    &day_02::Day02,
];
