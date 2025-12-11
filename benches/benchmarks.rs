use criterion::{Criterion, criterion_group, criterion_main};

macro_rules! benchmark {
    ($year:ident $( $day:ident ),* $(,)?) => {
        pub fn $year(c: &mut Criterion) {
            $(
                {
                    use aoc::$year::$day::*;
                    use aoc::common::input::*;
                    use aoc::common::parse::*;

                    let day: u32 = stringify!($day).parse_uint::<u32>();
                    let year: u32 = stringify!($year).parse_uint::<u32>();

                    if let Ok(data) = read_puzzle_input(&day, &year) {
                        let input = parse(&data);

                        let mut group = c.benchmark_group(
                            format!("{}::{}", stringify!($year), stringify!($day))
                        );

                        group.bench_function("both", |b| {
                            b.iter(|| {
                                let p1 = part1(std::hint::black_box(&input));
                                let p2 = part2(std::hint::black_box(&input));
                                std::hint::black_box((p1, p2));
                            });
                        });

                        group.finish();
                    }
                }
            )*
        }
    }
}

benchmark!(year2015
    day01, day02, day03, day04, day05, day06, day07, day08,
    day09, day10, day11, day12, day13, day14, day15, day16,
    day17, day18, day19, day20, day21, day22, day23, day24,
    day25
);

benchmark!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08,
    day09, day10, day11, day12, day13, day14, day15, day16,
    day17, day18, day19, day20, day21, day22, day23, day24,
    day25
);

benchmark!(year2025
    day01, day02, day03, day04, day05, day06, day07, day08,
    day09, day10
);

criterion_group!(year2015_group, year2015);
criterion_group!(year2024_group, year2024);
criterion_group!(year2025_group, year2025);

criterion_main!(year2015_group, year2024_group, year2025_group);
