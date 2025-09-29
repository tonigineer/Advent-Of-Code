macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(common "Utilities to solve Advent of Code."
    ansi, input
);

library!(year2015 "Help Santa by solving puzzles to fix the weather machine's snow function."
    day01, day02, day03, day04, day05, day06
);

library!(year2024 "Help Santa by solving puzzles to fix the weather machine's snow function."
    day01
);
