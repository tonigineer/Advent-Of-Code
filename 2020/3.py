""" https://adventofcode.com/2020/day/3 """

import aocd


def convertLines2Lists(data):
    """Convert string into a list (y) that holds all rows (x)."""
    dataAsLists = list()
    for line in data.splitlines():
        row = list()
        for item in line:
            row.append(item)
        dataAsLists.append(row)
    return dataAsLists


def solvePart1(dataAsLists, slope=(3, 1), startingPosition=(0, 0)):
    """Traverse down the pattern with wraparound for x."""
    treesEncountered = 0
    x = startingPosition[0]
    y = startingPosition[1]
    while y < len(dataAsLists)-1:
        x += slope[0]
        y += slope[1]
        x = x % len(dataAsLists[0])
        if dataAsLists[y][x] == '#':
            treesEncountered += 1
    return treesEncountered


def solvePart2(dataAsLists):
    """Simply run part1 for multiple slopes and multiply results."""
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    treesEncountered = 1
    for slope in slopes:
        treesEncountered *= solvePart1(dataAsLists, slope)
    return treesEncountered


if __name__ == "__main__":
    data = aocd.get_data(day=3, year=2020)
    dataAsLists = convertLines2Lists(data)
    print(f'Part 1: {solvePart1(dataAsLists)}')
    print(f'Part 2: {solvePart2(dataAsLists)}')
