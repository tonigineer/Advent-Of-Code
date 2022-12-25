# https://adventofcode.com/2022/day/25

FILENAME = './2022/25/25.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

total = 0

for line in lines:
    k = 1
    for c in line[::-1]:
        total += ("=-012".find(c) - 2) * k
        k *= 5

snafu = ''
while total:
    rem = total % 5
    total = total // 5

    if rem > 2:
        total += 1

    snafu = '012=-'[rem] + snafu

print(f'🎄 1. Solution: {snafu}')
