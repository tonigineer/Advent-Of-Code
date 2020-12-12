""" https://adventofcode.com/2020/day/10 """

import aocd
import numpy as np
from itertools import permutations, combinations


def solvePuzzle1(adapters):
    adapters.sort()
    diffs = np.diff(np.array(adapters))
    _, counts = np.unique(diffs, return_counts=True)
    return counts.prod()


def solvePuzzle2(adapters):
    adapters.sort()
    paths = {adapter: 0 for adapter in adapters}
    paths[0] += 1
    for adapter in adapters:
        for diff in range(1, 4):
            tempAdapter = adapter + diff
            if tempAdapter in adapters:
                paths[tempAdapter] += paths[adapter]
    return paths[max(adapters)]


if __name__ == "__main__":
    adapters = list(map(int, aocd.get_data(day=10, year=2020).splitlines()))
    adapters += [0, max(adapters)+3]  # add outlet and build-in adapter
    print(f'Part 1: {solvePuzzle1(adapters)}')
    print(f'Part 2: {solvePuzzle2(adapters)}')