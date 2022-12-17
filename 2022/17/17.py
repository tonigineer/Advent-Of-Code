# https://adventofcode.com/2022/day/17

FILENAME = './2022/17/17.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

FALLING_ROCKS = 2022
WIDTH = 7
NUM_PIECES = 5


def state():
    # how does it look from the top
    m = [-20] * WIDTH

    for (x, y) in solid:
        m[x] = max(m[x], y)

    # normalize
    top = max(m)
    return tuple([x - top for x in m])


def pieces(idx, y):
    return {
        0: set([(2, y), (3, y), (4, y), (5, y)]),
        1: set([(3, y+2), (2, y+1), (3, y+1), (4, y+1), (3, y)]),
        2: set([(2, y), (3, y), (4, y), (4, y+1), (4, y+2)]),
        3: set([(2, y), (2, y+1), (2, y+2), (2, y+3)]),
        4: set([(2, y), (3, y), (2, y+1), (3, y+1)])
    }[idx]


height = 0
ri = 0
ji = 0

jets = [1 if _ == '>' else -1 for _ in raw]
solid = set([(x, -1) for x in range(WIDTH)])

rock = pieces(ri % NUM_PIECES, height + 3)

while ri < FALLING_ROCKS:
    for jet in jets:

        moved = {(x + jet, y) for (x, y) in rock}
        if all(0 <= x < WIDTH for (x, _) in moved) and not (moved & solid):
            rock = moved

        moved = {(x, y-1) for (x, y) in rock}
        if moved & solid:
            solid |= rock

            height = max(y for (_, y) in solid)

            ri += 1
            if ri >= FALLING_ROCKS:
                break

            rock = pieces(ri % NUM_PIECES, height + 4)
        else:
            rock = moved

print(f'🎄 1. Solution: {height+1}')


height = 0
ri = 0
ji = 0

solid = set([(x, -1) for x in range(WIDTH)])

rock = pieces(ri % NUM_PIECES, height + 3)

seen = {}

FALLING_ROCKS = 1000000000000

while ri < FALLING_ROCKS:
    for ji, jet in enumerate(jets):

        moved = {(x + jet, y) for (x, y) in rock}
        if all(0 <= x < WIDTH for (x, _) in moved) and not (moved & solid):
            rock = moved

        moved = {(x, y-1) for (x, y) in rock}
        if moved & solid:
            solid |= rock

            height = max(y for (_, y) in solid)

            ri += 1
            o = height
            if ri >= FALLING_ROCKS:
                break

            rock = pieces(ri % NUM_PIECES, height + 4)

            # Remember state by looking from the top.
            # If state repeats, extrapolate and calculate
            # offset. Than continue till the end.
            key = (ji, ri % NUM_PIECES, state())
            if key in seen:
                lri, lh = seen[key]
                rem = FALLING_ROCKS - ri
                rep = rem // (ri - lri)

                # extrapolate
                off = rep * (height - lh)
                ri += rep * (ri - lri)

                seen = dict()
            seen[key] = (ri, height)
        else:
            rock = moved

print(f'🎅 2. Solution: {height+1 + off}')
