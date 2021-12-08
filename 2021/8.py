"""https://adventofcode.com/2021/day/8."""

from itertools import permutations


def part1(data):
    """Solution for first puzzle.

    Simply check length of values right of bar.
    """
    cnt = 0
    for line in data:
        _, values = line.split(' | ')
        for v in values.split():
            if len(v) in [2, 3, 4, 7]:
                cnt += 1
    return cnt


def part2(data):
    """Solution for first puzzle.

    Use permutations to look for the right mapping. Slow, but
    fast to implement.
    """
    cnt = 0
    truth_list = ['abcefg', 'cf', 'acdeg', 'acdfg', 'bcdf', 'abdfg', 'abdefg', 'acf', 'abcdefg', 'abcdfg']

    for line in data:
        digits, values = line.split(' | ')
        for perm in permutations('abcdefg'):
            new_mapping = {p: c for p, c in zip(perm, 'abcdefg')}
            translation = list()
            for digit in digits.split():
                digit_translated = "".join(sorted(map(new_mapping.get, digit)))
                translation.append(digit_translated)
            if set(translation) == set(truth_list):
                translated = ["".join(sorted(map(new_mapping.get, val))) for val in values.split()]
                cnt += int("".join(str(truth_list.index(digit)) for digit in translated))
    return cnt


if __name__ == "__main__":
    with open('data/8.in') as f:
        data = f.read().splitlines()

    print(f'Part 1: {part1(data)}')
    print(f'Part 2: {part2(data)}')
