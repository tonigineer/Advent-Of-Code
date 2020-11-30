"""https://adventofcode.com/2015/day/2."""
import aocd
import numpy as np


def part1(data):
    totalWrappingPaper = 0
    for dimensions in data:
        l, w, h = list(map(int, dimensions.split('x')))
        presentItself = 2*l*w + 2*w*h + 2*h*l
        extra = min([l*w, w*h, h*l])
        totalWrappingPaper += presentItself + extra
    return totalWrappingPaper


def part2(data):
    totalRibbon = 0
    for dimensions in data:
        l, w, h = list(map(int, dimensions.split('x')))
        s1, s2 = twoSmallestItems([l, w, h])
        sides = s1*2 + s2*2
        bow = l*w*h
        totalRibbon += sides + bow
    return totalRibbon

def twoSmallestItems(items:list):
    indices = np.argpartition(np.array(items), 1)
    return items[indices[0]], items[indices[1]]


if __name__ == "__main__":
    data = aocd.get_data(day=2, year=2015)
    data = list(map(str, data.splitlines()))
    print(f'Solution 1: {part1(data)}')
    print(f'Solution 2: {part2(data)}')
