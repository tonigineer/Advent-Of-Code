# https://adventofcode.com/2022/day/15

import re

FILENAME = './2022/15/15.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

TARGET_Y = 2_000_000
MAX_COORD = 4_000_000

sensors = set()
beacons = set()
cannot = set()

for line in lines:
    sx, sy, bx, by = map(int, re.findall('[0-9]+', line))
    d = abs(sx-bx) + abs(sy-by)
    sensors.add(((sx, sy), d))

    xo = d - abs(TARGET_Y-sy)
    if xo < 0:
        continue

    lx = sx - xo
    hx = sx + xo

    for x in range(lx, hx+1):
        cannot.add(x)

    if by == TARGET_Y:
        beacons.add(bx)

print(f'🎄 1. Solution: {len(cannot - beacons)}')


def seen_by_others(xn, yn):
    for s in sensors:
        (sx, sy), d = s
        dd = abs(xn-sx) + abs(yn-sy)
        if dd <= d:
            return True
    return False


for s in sensors:
    (sx, sy), d = s
    # loop over all edges of sensor within d+1
    for dx in range(d+2):
        dy = (d+1) - dx
        for sign_x, sign_y in [(1, -1), (-1, -1), (1, 1), (-1, 1)]:
            xn = sx + dx*sign_x
            yn = sy + dy*sign_y
            # assert abs(xn-sx)+abs(yn-sy) == d+1

            if not seen_by_others(xn, yn):
                print(f'🎅 2. Solution: {xn * MAX_COORD + yn}')
                exit(0)
