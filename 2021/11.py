"""https://adventofcode.com/2021/day/11."""


def part1(data, turns):
    """Solution for first and second puzzle."""
    flashes = 0
    for _ in range(turns):
        data = [[int(c) + 1 for c in r] for r in data]

        flashed = list()
        will_flash = list()

        for y, r in enumerate(data):
            for x, c in enumerate(r):
                if c > 9:
                    will_flash.append((y, x))

        while will_flash:
            r, c = will_flash.pop(0)
            if (r, c) in flashed or data[r][c] <= 9:
                continue
            flashed.append((r, c))
            for (y, x) in [(r, c+1), (r, c-1), (r+1, c), (r-1, c), (r+1, c+1), (r-1, c-1), (r-1, c+1), (r+1, c-1)]:
                if 0 <= x < 10 and 0 <= y < 10:
                    if (y, x) not in will_flash:
                        will_flash.append((y, x))
                    data[y][x] += 1

        for y, r in enumerate(data):
            for x, c in enumerate(r):
                if c > 9:
                    data[y][x] = 0

        flashes += len(flashed)
        if len(flashed) == 100:
            return _ + 1
    return flashes


if __name__ == "__main__":
    with open('data/11.in') as f:
        data = f.read().splitlines()

    print(f'Part 1: {part1(data, 100)}')
    print(f'Part 2: {part1(data, 50000)}')
