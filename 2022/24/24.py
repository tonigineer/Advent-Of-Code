# https://adventofcode.com/2022/day/24

import math
from collections import deque


FILENAME = './2022/24/24.in'

raw = open(FILENAME).read()
lines = [line.strip() for line in open(FILENAME)]

blizzards = tuple(set() for _ in range(4))

for y, line in enumerate(lines[1:]):
    for x, item in enumerate(line[1:]):
        if item in "<>^v":
            blizzards["<>^v".index(item)].add((x, y))

targets = [(x - 1, y), (0, -1)]
lcm = x * y // math.gcd(x, y)  # only performance gains

stage = 0
queue = deque([(0, (0, -1), stage)])  # time, position, stage
seen = set()

while queue:
    time, (cx, cy), _ = queue.popleft()
    time += 1

    for dx, dy in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]:
        nx, ny = cx + dx, cy + dy

        if (nx, ny) == targets[stage % 2]:
            if stage == 2:
                print(f'🎅 2. Solution: {time}')
                exit(0)
            if stage == 0:
                print(f'🎄 1. Solution: {time}')
            stage += 1
            seen = set()
            queue = deque([(time, (nx, ny), stage)])
            break

        if (nx, ny) not in targets and not (0 <= nx < x and 0 <= ny < y):
            continue

        fail = False
        if not (nx, ny) in targets:
            for bid, bdx, bdy in ((0, -1, 0), (1, 1, 0), (2, 0, -1), (3, 0, 1)):
                if ((nx - bdx * time) % x, (ny - bdy * time) % y) in blizzards[bid]:
                    fail = True
                    break
        if not fail:
            key = ((nx, ny), time % lcm)

            if key in seen:
                continue

            seen.add(key)
            queue.append((time, (nx, ny), stage))
