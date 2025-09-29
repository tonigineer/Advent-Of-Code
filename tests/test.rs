macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

test!(year2015
    day01, day02, day03, day04, day05, day06
);

test!(year2024
    day01
);
