# https://adventofcode.com/2022/day/14

FILENAME = './2022/14/14.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

M = set()

for line in lines:
    prev = None
    for coord in line.split('->'):
        x, y = map(int, coord.split(','))
        if prev:
            dx = x - prev[0]
            dy = y - prev[1]
            for i in range(max(abs(dx), abs(dy)) + 1):
                xx = prev[0] + (1 if dx > 0 else -1 if dx < 0 else 0) * i
                yy = prev[1] + (1 if dy > 0 else -1 if dy < 0 else 0) * i
                M.add((xx, yy))
        prev = x, y

bottom = max([p[1] for p in M])


def falling_sand(part):
    for sand in range(100_000):
        s = (500, 0)
        if s in M:
            return sand

        while True:
            if s[1] >= bottom and part == 1:
                return sand
            elif s[1] == bottom+1 and part == 2:
                break
            elif (s[0], s[1]+1) not in M:
                s = (s[0], s[1]+1)
            elif (s[0]-1, s[1]+1) not in M:
                s = (s[0]-1, s[1]+1)
            elif (s[0]+1, s[1]+1) not in M:
                s = (s[0]+1, s[1]+1)
            else:
                break
        M.add(s)


# or copy M again
part1 = falling_sand(1)
part2 = part1 + falling_sand(2)

print(f'🎄 1. Solution: {part1}')
print(f'🎅 2. Solution: {part2}')
