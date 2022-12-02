"""https://adventofcode.com/2021/day/13."""

import re


def part1(data, part2=False):
    """Solution for first and second puzzle.

    More list comprehension would be better :)
    """
    commands = [r for r in data if 'fold' in r]
    dots = [tuple(map(int, r.split(','))) for r in data if ',' in r]

    xmax = max([x for x, y in dots])
    ymax = max([y for x, y in dots])

    paper = [['.' for _ in range(xmax + 1)] for _ in range(ymax + 1)]

    for x, y in dots:
        paper[y][x] = '#'

    for command in commands:
        if 'y' in command:
            num = int([re.findall(r'\d+', command)][0][0])
            for y in range(num):
                for x in range(len(paper[0])):
                    if paper[-y-1][x] == '#':
                        paper[y][x] = '#'
            while len(paper) > num:
                paper.pop(num)

        if 'x' in command:
            num = int([re.findall(r'\d+', command)][0][0])
            for y in range(len(paper)):
                for x in range(len(paper[0])):
                    if paper[y][-x-1] == '#':
                        paper[y][x] = '#'
                while len(paper[y]) > num:
                    paper[y].pop(num)

        if not part2:
            break

    result = 0
    for r in paper:
        if part2:
            print(r)
        for c in r:
            if c == "#":
                result += 1

    return result


if __name__ == "__main__":
    with open('data/13.in') as f:
        data = f.read().splitlines()

    print(f'Part 1: {part1(data, False)}')
    print(f'Part 2:')
    part1(data, True)
