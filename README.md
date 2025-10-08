<h1 align="center">
  🎄 Advent of Code 🎄
</h1>

[![Tests](https://github.com/tonigineer/advent-of-code/actions/workflows/build.yml/badge.svg)](https://github.com/tonigineer/advent-of-code/actions/workflows/build.yml) [![Checks](https://github.com/tonigineer/advent-of-code/actions/workflows/clippy.yml/badge.svg)](https://github.com/tonigineer/advent-of-code/actions/workflows/clippy.yml) ![GitHub last commit](https://img.shields.io/github/last-commit/tonigineer/advent-of-code)

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

make format                   # Run Clippy for linting
```

Input files are downloaded automatically. To enable this, create a file named `adventofcode.session` in the project root containing your Advent of Code session cookie.

### [2024](https://adventofcode.com/2024) — 50 ⭐

[![CI](https://github.com/tonigineer/advent-of-code/actions/workflows/test2024.yml/badge.svg)](https://github.com/tonigineer/advent-of-code/actions/workflows/test2024.yml)

| Day                             | Day                             | Day                             |
| ------------------------------- | ------------------------------- | ------------------------------- |
| [Day 01](src/year2024/day01.rs) | [Day 02](src/year2024/day02.rs) | [Day 03](src/year2024/day03.rs) |
| [Day 04](src/year2024/day04.rs) | [Day 05](src/year2024/day05.rs) | [Day 06](src/year2024/day06.rs) |
| [Day 07](src/year2024/day07.rs) | [Day 08](src/year2024/day08.rs) | [Day 09](src/year2024/day09.rs) |
| [Day 10](src/year2024/day10.rs) | [Day 11](src/year2024/day11.rs) | [Day 12](src/year2024/day12.rs) |
| [Day 13](src/year2024/day13.rs) | [Day 14](src/year2024/day14.rs) | [Day 15](src/year2024/day15.rs) |
| [Day 16](src/year2024/day16.rs) | [Day 17](src/year2024/day17.rs) | [Day 18](src/year2024/day18.rs) |
| [Day 19](src/year2024/day19.rs) | [Day 20](src/year2024/day20.rs) | [Day 21](src/year2024/day21.rs) |
| [Day 22](src/year2024/day22.rs) | [Day 23](src/year2024/day23.rs) | [Day 24](src/year2024/day24.rs) |
| [Day 25](src/year2024/day25.rs) |                                 |                                 |

### [2015](https://adventofcode.com/2015) — 50 ⭐

[![CI](https://github.com/tonigineer/advent-of-code/actions/workflows/test2015.yml/badge.svg)](https://github.com/tonigineer/advent-of-code/actions/workflows/test2015.yml)

| Day                             | Day                             | Day                             |
| ------------------------------- | ------------------------------- | ------------------------------- |
| [Day 01](src/year2015/day01.rs) | [Day 02](src/year2015/day02.rs) | [Day 03](src/year2015/day03.rs) |
| [Day 04](src/year2015/day04.rs) | [Day 05](src/year2015/day05.rs) | [Day 06](src/year2015/day06.rs) |
| [Day 07](src/year2015/day07.rs) | [Day 08](src/year2015/day08.rs) | [Day 09](src/year2015/day09.rs) |
| [Day 10](src/year2015/day10.rs) | [Day 11](src/year2015/day11.rs) | [Day 12](src/year2015/day12.rs) |
| [Day 13](src/year2015/day13.rs) | [Day 14](src/year2015/day14.rs) | [Day 15](src/year2015/day15.rs) |
| [Day 16](src/year2015/day16.rs) | [Day 17](src/year2015/day17.rs) | [Day 18](src/year2015/day18.rs) |
| [Day 19](src/year2015/day19.rs) | [Day 20](src/year2015/day20.rs) | [Day 21](src/year2015/day21.rs) |
| [Day 22](src/year2015/day22.rs) | [Day 23](src/year2015/day23.rs) | [Day 24](src/year2015/day24.rs) |
| [Day 25](src/year2015/day25.rs) |                                 |                                 |
