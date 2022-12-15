# https://adventofcode.com/2022/day/15

import re

FILENAME = './2022/15/15.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

y = 2000000
X = set(list(range(-5_000_000, 5_000_000)))
len_start = len(X)

B = set()
S = set()

for line in lines:
    sx, sy, bx, by = map(int, re.findall('[0-9]+', line))
    d = abs(sx-bx) + abs(sy-by)
    S.add(((sx, sy), d))

for s in S:
    (sx, sy), d = s
    if by == y:
        B.add((bx, by))
    to_be_removed = []
    for x in X:
        if abs(x-sx) + abs(y-sy) <= d:
            to_be_removed.append(x)
    for x in to_be_removed:
        X.remove(x)

print(f'🎄 1. Solution: {len_start - len(X) - len(B)}')


def seen_by_others(xn, yn):
    if not 0 <= xn <= 4_000_000 and 0 <= yn <= 4_000_000:
        return False
    for s in S:
        (sx, sy), d = s
        dd = abs(xn-sx) + abs(yn-sy)
        if dd <= d:
            return True
    return False


for s in S:
    (sx, sy), d = s
    for dx in range(d+2):
        dy = (d+1) - dx
        for sign_x, sign_y in [(1, -1), (-1, -1), (1, 1), (-1, 1)]:
            xn = sx + dx*sign_x
            yn = sy + dy*sign_y
            # assert abs(xn-sx)+abs(yn-sy) == d+1
            if not seen_by_others(xn, yn):
                print(f'🎅 2. Solution: {xn * 4_000_000 + yn}')
                exit(0)
