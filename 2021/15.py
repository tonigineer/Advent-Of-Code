"""https://adventofcode.com/2021/day/15."""

import heapq

def wrap(x):
    while x > 9:
        x -= 9
    return x

with open('data/15.in') as f:
    lines = f.read().splitlines()

grid = [list(map(int, line)) for line in lines]

r_max = len(grid) * 5
c_max = len(grid[0]) * 5

new_grid = [[0] * r_max for _ in range(c_max)]

for r in range(r_max):
    for c in range(c_max):
        new_grid[r][c] = wrap(grid[r % len(grid)][c % len(grid[0])] + r // len(grid) + c // len(grid[0]))

grid = new_grid
paths = [(0, 0, 0)]
visited = list()

while True:
    val, x, y = heapq.heappop(paths)
    if (x, y) in visited:
        continue
    if (x, y) == (len(grid) - 1, len(grid[0]) - 1):
        break
    visited.append((x, y))
    for nx, ny in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
        if (nx, ny) in visited:
            continue
        if len(grid) > nx >= 0 <= ny < len(grid[0]):
            heapq.heappush(paths, (val + grid[nx][ny], nx, ny))

print(val)
