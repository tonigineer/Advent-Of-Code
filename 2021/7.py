"""https://adventofcode.com/2021/day/7."""


def intrange(n):
    """Get sum of integer range."""
    return int(n*(n+1)/2)


def part1(data):
    """Solution for first puzzle.

    Sum distance of all positions to median.
    """
    positions = sorted(list(map(int, data.split(','))))
    median = positions[(len(positions)) // 2]
    return sum([abs(median - position) for position in positions])


def part2(data):
    """Solution to second puzzle.

    Brute force solution with checking all combinations.
    """
    fuel_spent = list()
    positions = list(map(int, data.split(',')))
    for target in range(min(positions), max(positions)+1):
        fuel_spent.append(sum([intrange(abs(target-position)) for position in positions]))
    return min(fuel_spent)


if __name__ == "__main__":
    with open('data/7.in') as f:
        data = f.read()

    print(f'Part 1: {part1(data)}')
    print(f'Part 2: {part2(data)}')
