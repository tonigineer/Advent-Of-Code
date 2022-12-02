"""https://adventofcode.com/2021/day/3."""


def part1(data):
    """Solve first part of puzzle."""
    instructions = len(data)
    gamma = ''

    for bit in range(len(data[0])-1, -1, -1):
        gamma += '1' if len([c for c in data if int(c, 2) & (1 << bit)]) > instructions/2 else '0'

    epsilon = int(gamma.replace('1', '2').replace('0', '1').replace('2', '0'), 2)
    gamma = int(gamma, 2)
    print(f'Part 1: {gamma * epsilon}')


def part2(data_orig):
    """Solve second part of puzzle."""
    data = data_orig
    for bit in range(len(data[0])-1, -1, -1):
        data_1 = [c for c in data if int(c, 2) & (2 ** bit)]
        data_0 = [c for c in data if not int(c, 2) & (2 ** bit)]
        data = data_1 if len(data_1) >= len(data_0) else data_0

        if len(data) == 1:
            gamma = int(data[0], 2)
            break

    data = data_orig
    for bit in range(len(data[0])-1, -1, -1):
        data_1 = [c for c in data if int(c, 2) & (2 ** bit)]
        data_0 = [c for c in data if not int(c, 2) & (2 ** bit)]
        data = data_0 if len(data_0) <= len(data_1) else data_1

        if len(data) == 1:
            epsilon = int(data[0], 2)
            break

    print(f'Part 2: {gamma * epsilon}')


if __name__ == "__main__":
    with open('data/3.in') as f:
        data = f.read().splitlines()

    part1(data)
    part2(data)
