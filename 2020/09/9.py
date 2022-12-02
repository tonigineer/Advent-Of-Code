""" https://adventofcode.com/2020/day/9 """

import aocd
import itertools


def checkSeries(data):
    """Check if sum of two items within window are equal to current value."""
    for item in itertools.combinations(data[:-1], 2):
        if sum(list(map(int, item))) == int(data[-1]):
            return True


def findRangeToFitValue(data, value, is_=0, ie_=1):
    """Find contiguous  range summed up match value."""
    sum_ = data[is_] + data[ie_]
    while sum_ != value:
        while sum_ < value:
            ie_ += 1
            sum_ += data[ie_]
        while sum_ > value:
            sum_ -= data[is_]
            is_ += 1
    return min(data[is_:ie_]) + max(data[is_:ie_])


def solvePuzzle1(data, windowSize=25):
    """Loop through data series. and check series."""
    for idx in range(windowSize, len(data)):
        if not checkSeries(data[idx-windowSize:idx+1]):
            return data[idx]


def solvePuzzle2(data):
    """Get target value from first puzzle and find range."""
    return findRangeToFitValue(data, solvePuzzle1(data))


if __name__ == "__main__":
    data = list(map(int, aocd.get_data(day=9, year=2020).splitlines()))
    print(f'Part 1: {solvePuzzle1(data)}')
    print(f'Part 2: {solvePuzzle2(data)}')
