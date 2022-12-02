"""https://adventofcode.com/2021/day/5."""


def part(data, consider_diagonals=False):
    """Count overlapping coordinates from lines."""
    occupancy_grid = dict()
    for line in data:
        x1, y1, x2, y2 = map(int, line.replace(' -> ', ', ').split(','))

        if x1 != x2 and y1 != y2 and consider_diagonals is False:
            continue

        dx = x2 - x1 if x2 > x1 else x1 - x2
        dy = y2 - y1 if y2 > y1 else y1 - y2

        x_dir = 1 if x2 > x1 else -1
        y_dir = 1 if y2 > y1 else -1

        if x1 == x2:
            x_dir = 0
        if y1 == y2:
            y_dir = 0

        coords = list()
        for _ in range(0, max(dx, dy)+1):
            coords.append((x1, y1))
            x1 += x_dir
            y1 += y_dir

        for x, y in coords:
            key = f'{x}-{y}'
            if key not in occupancy_grid.keys():
                occupancy_grid[key] = 1
            else:
                occupancy_grid[key] += 1

    return len([_ for _ in occupancy_grid.keys() if occupancy_grid[_] > 1])


if __name__ == "__main__":
    with open('data/5.in') as f:
        data = f.read().splitlines()

    print(f'Part 1: {part(data, False)}')
    print(f'Part 2: {part(data, True)}')
