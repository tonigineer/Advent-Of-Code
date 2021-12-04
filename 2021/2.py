"""https://adventofcode.com/2021/day/2."""


def part1(data: list):
    """Solve first part of puzzle."""
    x, d = 0, 0
    for cmd in data:
        direction, val = cmd.split(' ')
        val = int(val)
        if direction == 'forward':
            x += val
        if direction == 'down':
            d += val
        if direction == 'up':
            d -= val

    print(f'Part 1: {x*d}')


def part2(data: list):
    """Solve second part of puzzle."""
    x, d, aim = 0, 0, 0

    for cmd in data:
        direction, val = cmd.split(' ')
        val = int(val)
        if direction == 'forward':
            x += val
            d += aim*val
        if direction == 'down':
            aim += val
        if direction == 'up':
            aim -= val

    print(f'Part 2: {x*d}')


if __name__ == "__main__":
    with open('data/2.in') as f:
        data = f.read().splitlines()

    part1(data)
    part2(data)
