use criterion::{Criterion, criterion_group, criterion_main};

macro_rules! benchmark {
    ($year:ident $( $day:ident ),* $(,)?) => {
        pub fn $year(c: &mut Criterion) {
            $(
                {
                    use aoc::$year::$day::*;
                    use aoc::common::input::*;

                    let day: u32 = stringify!($day)["day".len()..].parse().unwrap();
                    let year: u32 = stringify!($year)["year".len()..].parse().unwrap();

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
    day01, day02
);

benchmark!(year2024
    day01,
);

criterion_group!(year2015_group, year2015);
criterion_group!(year2024_group, year2024);

criterion_main!(year2015_group, year2024_group);
