# https://adventofcode.com/2022/day/22

import re

FILENAME = './2022/22/22.in'

raw = open(FILENAME).read()
lines = [line.strip() for line in open(FILENAME)]

max_width = max([len(line) for line in lines]) + 1

grid = []
sequence = lines[-1]

for line in raw.split('\n')[:-2]:
    grid.append(line + ' ' * (max_width - len(line)))

y, x = 0, 0
while grid[y][x] != '.':
    x += 1

dx = 1
dy = 0

for moves, turn in re.findall(r"(\d+)([RL]?)", sequence):
    m = int(moves)

    for _ in range(m):
        nx = x
        ny = y

        while True:
            nx = (nx + dx) % len(grid[0])
            ny = (ny + dy) % len(grid)
            if grid[ny][nx] != ' ':
                break

        if grid[ny][nx] == '#':
            break

        x, y = nx, ny

    if turn == 'R':
        dx, dy = -dy, dx
    elif turn == 'L':
        dx, dy = dy, -dx

heading_score = {'10': 0, '01': 1, '-10': 2, '0-1': 3}[f'{dx}{dy}']
print(f'🎄 1. Solution: {4 * (x+1) + 1000 * (y+1) + heading_score}')


y, x = 0, 0
while grid[y][x] != '.':
    x += 1

dx = 1
dy = 0

for moves, turn in re.findall(r"(\d+)([RL]?)", sequence):
    m = int(moves)

    for _ in range(m):
        cdy = dy
        cdx = dx

        ny = y + dy
        nx = x + dx

        if ny < 0 and 50 <= nx < 100 and dy == -1:
            dy, dx = 0, 1
            ny, nx = nx + 100, 0
        elif nx < 0 and 150 <= ny < 200 and dx == -1:
            dy, dx = 1, 0
            ny, nx = 0, ny - 100
        elif ny < 0 and 100 <= nx < 150 and dy == -1:
            ny, nx = 199, nx - 100
        elif ny >= 200 and 0 <= nx < 50 and dy == 1:
            ny, nx = 0, nx + 100
        elif nx >= 150 and 0 <= ny < 50 and dx == 1:
            dx = -1
            ny, nx = 149 - ny, 99
        elif nx == 100 and 100 <= ny < 150 and dx == 1:
            dx = -1
            ny, nx = 149 - ny, 149
        elif ny == 50 and 100 <= nx < 150 and dy == 1:
            dy, dx = 0, -1
            ny, nx = nx - 50, 99
        elif nx == 100 and 50 <= ny < 100 and dx == 1:
            dy, dx = -1, 0
            ny, nx = 49, ny + 50
        elif ny == 150 and 50 <= nx < 100 and dy == 1:
            dy, dx = 0, -1
            ny, nx = nx + 100, 49
        elif nx == 50 and 150 <= ny < 200 and dx == 1:
            dy, dx = -1, 0
            ny, nx = 149, ny - 100
        elif ny == 99 and 0 <= nx < 50 and dy == -1:
            dy, dx = 0, 1
            ny, nx = nx + 50, 50
        elif nx == 49 and 50 <= ny < 100 and dx == -1:
            dy, dx = 1, 0
            ny, nx = 100, ny - 50
        elif nx == 49 and 0 <= ny < 50 and dx == -1:
            dx = 1
            ny, nx = 149 - ny, 0
        elif nx < 0 and 100 <= ny < 150 and dx == -1:
            dx = 1
            ny, nx = 149 - ny, 50
        else:
            pass

        if grid[ny][nx] == "#":
            dy = cdy
            dx = cdx
            break

        x, y = nx, ny

    if turn == 'R':
        dx, dy = -dy, dx
    elif turn == 'L':
        dx, dy = dy, -dx

heading_score = {'10': 0, '01': 1, '-10': 2, '0-1': 3}[f'{dx}{dy}']
print(f'🎅 2. Solution: {4 * (x+1) + 1000 * (y+1) + heading_score}')
