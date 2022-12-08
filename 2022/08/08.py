# https://adventofcode.com/2022/day/8


lines = [line.strip() for line in open('./2022/08/08.in')]

grid = [list(map(int, row)) for row in lines]

w = len(grid[0])
h = len(grid)


ans = 0
for r in range(h):
    for c in range(w):
        t = grid[r][c]

        # up
        if r == 0 or max([grid[y][c] for y in range(r-1, -1, -1)]) < t:
            ans += 1

        # down
        elif r == h-1 or max([grid[y][c] for y in range(r+1, h)]) < t:
            ans += 1

        # left
        elif c == 0 or max([grid[r][x] for x in range(c-1, -1, -1)]) < t:
            ans += 1

        # right
        elif c == w-1 or max([grid[r][x] for x in range(c+1, w)]) < t:
            ans += 1

print(f'🎄 1. Solution: {ans}')


scores = {}
for r in range(h):
    for c in range(w):
        if r == 0 or r == h-1 or c == 0 or c == w-1:
            scores[(r, c)] = 0
            continue

        t = grid[r][c]

        # up
        m1 = 1
        for tt in [grid[y][c] for y in range(r-1, -1, -1)][:-1]:
            if tt < t:
                m1 += 1
            else:
                break

        m2 = 1
        for tt in [grid[y][c] for y in range(r+1, h)][:-1]:
            if tt < t:
                m2 += 1
            else:
                break

        # left
        m3 = 1
        for tt in [grid[r][x] for x in range(c-1, -1, -1)][:-1]:
            if tt < t:
                m3 += 1
            else:
                break

        # right
        m4 = 1
        for tt in [grid[r][x] for x in range(c+1, w)][:-1]:
            if tt < t:
                m4 += 1
            else:
                break

        scores[(r, c)] = m1 * m2 * m3 * m4

print(f'🎅 2. Solution: {max(scores.values())}')
