# https://adventofcode.com/2022/day/20

from collections import deque

FILENAME = './2022/20/20.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]


def solve(part):
    num = list(map(int, lines))
    num = [v * (811589153 if part == 2 else 1) for v in num]
    num = deque(enumerate(num))

    for _ in range(1 if part == 1 else 10):
        for n in range(len(num)):
            for i in range(len(num)):
                if num[i][0] == n:
                    break

            while num[0][0] != n:
                num.append(num.popleft())
            val = num.popleft()

            shift = val[1] % len(num)

            for _ in range(shift):
                num.append(num.popleft())
            num.append(val)

    for i in range(len(num)):
        if num[i][1] == 0:
            return num[(i+1000) % len(num)][1] + \
                num[(i+2000) % len(num)][1] + \
                num[(i+3000) % len(num)][1]


print(f'🎄 1. Solution: {solve(1)}')
print(f'🎅 2. Solution: {solve(2)}')
