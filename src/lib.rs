macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(common "Utilities to solve Advent of Code."
    ansi, grid, grid_legacy, input, integer, parse, position
);

library!(year2015 "Help Santa by solving puzzles to fix the weather machine's snow function."
    day01, day02, day03, day04, day05, day06, day07, day08,
    day09, day10, day11, day12, day13, day14, day15, day16,
    day17, day18, day19, day20, day21, day22, day23, day24,
    day25
);

library!(year2024 "Locate the Chief Historian in time for the big Christmas sleigh launch."
    day01, day02, day03, day04, day05, day06, day07, day08,
    day09, day10, day11, day12, day13, day14, day15, day16,
    day17, day18, day19, day20, day21, day22, day23, day24,
    day25
);

library!(year2025 "Finish decorating the North Pole by December 12th."
    day01, day02, day03, day04, day05, day06, day07, day08,
    day09, day10, day11
);
