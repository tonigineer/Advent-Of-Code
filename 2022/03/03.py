# https://adventofcode.com/2022/day/3

lines = [line.strip() for line in open('2022/03/03.in')]

res = 0

for line in lines:
    comp1, comp2 = line[:len(line)//2], line[len(line)//2:]
    match = set(comp1) & set(comp2)
    match = match.pop()
    if match.isupper():
        res += ord(match)-38
    else:
        res += ord(match)-96

print(f'1. Solution: {res}')

res = 0
for i in range(0, len(lines), 3):
    bag1, bag2, bag3 = lines[i:i+3]
    match = set(bag1) & set(bag2) & set(bag3)
    match = match.pop()
    if match.isupper():
        res += ord(match)-38
    else:
        res += ord(match)-96

print(f'2. Solution: {res}')
