"""https://adventofcode.com/2021/day/9."""


from collections import deque


def part1(data):
    """Solution for first puzzle.

    Pretty straight forward, i guess.
    """
    result = 0

    for r in range(len(data)):
        for c in range(len(data[0])):
            adjacent_values = list()

            for y, x in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]:
                if x < 0 or x >= len(data[0]) or y < 0 or y >= len(data):
                    continue
                adjacent_values.append(int(data[y][x]))

            current_value = int(data[r][c])

            if current_value < min(adjacent_values):
                result += current_value + 1
    return result


def part2(data):
    """Solution for second puzzle.

    Finding basins by checking for rising adjacent tiles next to
    each other.
    """
    basin_sizes = list()
    visited = list()

    for rr in range(len(data)):
        for cc in range(len(data[0])):
            if int(data[rr][cc]) == 9:
                continue

            size = 1
            positions = deque()
            positions.append((rr, cc))
            visited.append((rr, cc))

            while positions:
                r, c = positions.popleft()
                for y, x in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]:
                    if 0 <= x < len(data[0]) and 0 <= y < len(data):
                        if (y, x) in visited or int(data[y][x]) == 9:
                            continue
                        size += 1
                        positions.append((y, x))
                        visited.append((y, x))
            basin_sizes.append(size)

    basin_sizes = sorted(basin_sizes)
    return basin_sizes[-3] * basin_sizes[-2] * basin_sizes[-1]


if __name__ == "__main__":
    with open('data/9.in') as f:
        data = f.read().splitlines()

    print(f'Part 1: {part1(data)}')
    print(f'Part 2: {part2(data)}')
