# https://adventofcode.com/2022/day/4


lines = [line.strip() for line in open('2022/04/04.in')]

ans = 0
ans2 = 0

for line in lines:
    a, b = line.split(',')
    s1, e1 = map(int, a.split('-'))
    s2, e2 = map(int, b.split('-'))

    if s1 <= s2 and e2 <= e1 or s2 <= s1 and e1 <= e2:
        ans += 1

    if s1 <= s2 <= e1 or s1 <= e2 <= e1 or s2 <= s1 <= e2 or s2 <= e1 <= e2:
        ans2 += 1

print(f'1. Solution: {ans}')
print(f'2. Solution: {ans2}')
