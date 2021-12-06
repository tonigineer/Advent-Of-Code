"""https://adventofcode.com/2021/day/6."""

REBORN_LIFESPAN = 6
BORN_LIFESPAN = 8


def part(data, days):
    """Solution to first and second puzzle.

    Key is to keep track of number of fishes and not
    every single fish.
    """
    fish_level = [0 for _ in range(0, BORN_LIFESPAN+1)]
    for fish in list(map(int, data.split(','))):
        fish_level[fish] += 1

    for _ in range(0, days):
        died = fish_level.pop(0)
        fish_level[REBORN_LIFESPAN] += died
        fish_level.append(died)

    return sum(fish_level)


if __name__ == "__main__":
    with open('data/6.in') as f:
        data = f.read()

    print(f'Part 1: {part(data, 80)}')
    print(f'Part 2: {part(data, 256)}')
