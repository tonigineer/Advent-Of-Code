""" https://adventofcode.com/2020/day/1 """

import aocd
import itertools
import numpy as np


def solvePuzzle(numbers, depth, targetValue=2020):
    """Generic approach to solve puzzle."""
    for values in itertools.combinations(numbers, depth):
        if sum(values) == targetValue:
            return np.prod(values)


if __name__ == "__main__":
    data = aocd.get_data(day=1, year=2020)
    numbers = list(map(int, data.splitlines()))
    print(f'Part 1: {solvePuzzle(numbers, 2)}')
    print(f'Part 2: {solvePuzzle(numbers, 3)}')
