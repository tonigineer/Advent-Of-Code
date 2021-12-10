"""https://adventofcode.com/2021/day/10."""


from statistics import median


ERR_VALUES = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137
}

POINTS = {
    '(': 1,
    '[': 2,
    '{': 3,
    '<': 4
}

OPEN = ['(', '[', '{', '<']
CLOSE = [')', ']', '}', '>']


def part1(data):
    """Solution for first puzzle.

    Simply checking and removing close brackets
    from opened list. Also preparing data for
    part 2.
    """
    result = list()
    data_part2 = list()
    for line in data:
        opened = list()
        for c in line:
            if c in OPEN:
                opened.append(c)
                continue
            if CLOSE.index(c) != OPEN.index(opened.pop(-1)):
                result.append(ERR_VALUES[c])
                opened = None
                break
        if opened:
            data_part2.append("".join(opened))

    return sum(result), data_part2


def part2(data):
    """Solution for second puzzle.

    Adapted points dict to open characters, but
    OPENEND list must be reversed.
    """
    scores = list()
    for line in data:
        score = 0
        for c in reversed(line):
            score = score*5 + POINTS[c]
        scores.append(score)
    return median(sorted(scores))


if __name__ == "__main__":
    with open('data/10.in') as f:
        data = f.read().splitlines()

    restult_part1, data_part2 = part1(data)
    print(f'Part 1: {restult_part1}')
    print(f'Part 2: {part2(data_part2)}')
