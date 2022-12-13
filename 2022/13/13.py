# https://adventofcode.com/2022/day/13

from functools import cmp_to_key


FILENAME = './2022/13/13.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]


def check(left, right):
    if isinstance(left, int) and isinstance(right, int):
        if left == right:
            return 0
        return 1 if left < right else -1

    elif isinstance(left, list) and isinstance(right, list):
        i = 0
        while i < len(left) and i < len(right):
            c = check(left[i], right[i])
            if c != 0:
                return c
            i += 1

        if len(left) == len(right):
            return 0
        return 1 if len(left) < len(right) else -1

    elif isinstance(left, list) and isinstance(right, int):
        return check(left, [right])
    else:
        return check([left], right)


groups = raw.split('\n\n')

packets = [[[2]], [[6]]]
ans = 0
for idx, group in enumerate(groups):
    left, right = group.split('\n')
    left = eval(left)
    right = eval(right)

    if check(left, right) == 1:
        ans += idx + 1

    packets.append(left)
    packets.append(right)


print(f'🎄 1. Solution: {ans}')


packets = sorted(
    packets, key=cmp_to_key(lambda l, r: check(l, r)), reverse=True
)
ans2 = (packets.index([[2]])+1) * (packets.index([[6]])+1)

print(f'🎅 2. Solution: {ans2}')
