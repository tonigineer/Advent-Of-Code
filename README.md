<h1 align="center">
  🎄 Advent of Code 🎄
</h1>

[![Rust](https://img.shields.io/badge/Rust-DEA584?style=flat&logo=Rust&logoColor=black)](https://www.rust-lang.org/)
[![Tests](https://github.com/tonigineer/advent-of-code/actions/workflows/tests.yml/badge.svg)](https://github.com/tonigineer/advent-of-code/actions/workflows/tests.yml)
[![Checks](https://github.com/tonigineer/advent-of-code/actions/workflows/checks.yml/badge.svg)](https://github.com/tonigineer/advent-of-code/actions/workflows/checks.yml) 
![GitHub last commit](https://img.shields.io/github/last-commit/tonigineer/advent-of-code)
![Repo Size](https://img.shields.io/github/repo-size/tonigineer/Advent-Of-Code)
![Badge](https://hitscounter.dev/api/hit?url=https%3A%2F%2Fgithub.com%2Ftonigineer22%2FAdvent-Of-Code&label=Hits&icon=github&color=%2320c997&message=&style=flat&tz=UTC)

> <cite> [Advent of Code](https://adventofcode.com/)
>
> Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like. People use them as interview prep, company training, university coursework, practice problems, a speed contest, or to challenge each other.</cite>

## How to

All necessary Cargo commands are wrapped in a convenient **Makefile**. During the event, the `make` command is especially handy for quickly testing and solving each day's puzzle.

```bash
make                          # Test and solve today's puzzle automatically
make new                      # Create boilerplate files for today's DAY and YEAR

make 03                       # Test + solve Day 03 (for current year)
make test DAY=05              # Run only tests for Day 05 (for current year)
make solve YEAR=2023 DAY=14   # Solve Day 14 for Year 2023
make bench YEAR=2024          # Benchmark entire Year

make checks                   # Run clippy and fmt with cargo
```

Input files are downloaded automatically. To enable this, create a file named
`adventofcode.session` in the project root containing your Advent of Code session
cookie. Timings are `cargo bench` averages measured on an Intel Core i9-13900K.

### [2025](https://adventofcode.com/2025) — 6 ⭐

| Day                                         | Day                                         | Day                                         |
| ------------------------------------------- | ------------------------------------------- | ------------------------------------------- |
| [Day 01](src/year2025/day01.rs) [88.289 µs]  | [Day 02](src/year2025/day02.rs) [136.87 ms] | [Day 03](src/year2025/day03.rs) [54.877 µs] |



### [2024](https://adventofcode.com/2024) — 50 ⭐

| Day                                         | Day                                         | Day                                         |
| ------------------------------------------- | ------------------------------------------- | ------------------------------------------- |
| [Day 01](src/year2024/day01.rs) [37.42 µs]  | [Day 02](src/year2024/day02.rs) [331.92 µs] | [Day 03](src/year2024/day03.rs) [387.15 µs] |
| [Day 04](src/year2024/day04.rs) [707.99 µs] | [Day 05](src/year2024/day05.rs) [502.97 µs] | [Day 06](src/year2024/day06.rs) [4.00 s]    |
| [Day 07](src/year2024/day07.rs) [1.36 ms]   | [Day 08](src/year2024/day08.rs) [153.71 µs] | [Day 09](src/year2024/day09.rs) [29.94 ms]  |
| [Day 10](src/year2024/day10.rs) [446.78 µs] | [Day 11](src/year2024/day11.rs) [245.31 ns] | [Day 12](src/year2024/day12.rs) [112.75 ms] |
| [Day 13](src/year2024/day13.rs) [68.61 µs]  | [Day 14](src/year2024/day14.rs) [62.28 µs]  | [Day 15](src/year2024/day15.rs) [3.74 ms]   |
| [Day 16](src/year2024/day16.rs) [1.50 ns]   | [Day 17](src/year2024/day17.rs) [1.99 µs]   | [Day 18](src/year2024/day18.rs) [42.21 ms]  |
| [Day 19](src/year2024/day19.rs) [29.82 ms]  | [Day 20](src/year2024/day20.rs) [1.29 ns]   | [Day 21](src/year2024/day21.rs) [16.51 µs]  |
| [Day 22](src/year2024/day22.rs) [41.99 ms]  | [Day 23](src/year2024/day23.rs) [5.21 µs]   | [Day 24](src/year2024/day24.rs) [18.34 µs]  |
| [Day 25](src/year2024/day25.rs) [1.34 ms]   |                                             |                                             |



### [2015](https://adventofcode.com/2015) — 50 ⭐

| Day                                         | Day                                         | Day                                         |
| ------------------------------------------- | ------------------------------------------- | ------------------------------------------- |
| [Day 01](src/year2015/day01.rs) [12.58 µs]  | [Day 02](src/year2015/day02.rs) [91.58 µs]  | [Day 03](src/year2015/day03.rs) [602.51 µs] |
| [Day 04](src/year2015/day04.rs) [1.02 s]    | [Day 05](src/year2015/day05.rs) [1.37 ns]   | [Day 06](src/year2015/day06.rs) [42.49 ms]  |
| [Day 07](src/year2015/day07.rs) [256.92 µs] | [Day 08](src/year2015/day08.rs) [172.80 µs] | [Day 09](src/year2015/day09.rs) [11.84 ms]  |
| [Day 10](src/year2015/day10.rs) [251.81 ms] | [Day 11](src/year2015/day11.rs) [12.75 ms]  | [Day 12](src/year2015/day12.rs) [2.35 ms]   |
| [Day 13](src/year2015/day13.rs) [282.87 ms] | [Day 14](src/year2015/day14.rs) [30.92 µs]  | [Day 15](src/year2015/day15.rs) [231.61 ms] |
| [Day 16](src/year2015/day16.rs) [33.49 µs]  | [Day 17](src/year2015/day17.rs) [13.66 ms]  | [Day 18](src/year2015/day18.rs) [270.21 ms] |
| [Day 19](src/year2015/day19.rs) [501.76 µs] | [Day 20](src/year2015/day20.rs) [50.88 ms]  | [Day 21](src/year2015/day21.rs) [14.78 µs]  |
| [Day 22](src/year2015/day22.rs) [7.67 ms]   | [Day 23](src/year2015/day23.rs) [4.39 µs]   | [Day 24](src/year2015/day24.rs) [7.53 ms]   |
| [Day 25](src/year2015/day25.rs) [50.10 ms]  |                                             |                                             |
