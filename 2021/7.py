"""https://adventofcode.com/2021/day/7."""


def part1(data):
    """Solution for first puzzle.

    Simply (brute force) loop over all position and check against
    complete range from min and max of positions.
    """
    fuel_spent = list()
    positions = list(map(int, data.split(',')))
    for target in range(min(positions), max(positions)+1):
        fuel_spent.append(sum([abs(target-position) for position in positions]))
    return min(fuel_spent)


def part2(data):
    """Solution to second puzzle.

    Same as part1, except for using sum from 0 to diff instead of diff.
    """
    fuel_spent = list()
    positions = list(map(int, data.split(',')))
    for target in range(min(positions), max(positions)+1):
        fuel_spent.append(sum([sum(range(0, abs(target-position)+1)) for position in positions]))
    return min(fuel_spent)


if __name__ == "__main__":
    with open('data/7.in') as f:
        data = f.read()

    print(f'Part 1: {part1(data)}')
    print(f'Part 2: {part2(data)}')
