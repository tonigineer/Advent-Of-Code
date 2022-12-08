# https://adventofcode.com/2022/day/7

from collections import defaultdict

lines = [line.strip() for line in open('./2022/07/07.in')]

tree = defaultdict(int)
pwd = []

for line in lines:
    parts = line.strip().split()
    if parts[1] == 'cd':
        if parts[2] == '..':
            pwd.pop()
        else:
            pwd.append(parts[2])
    elif parts[0] == 'dir' or parts[1] == 'ls':
        continue
    else:
        size = int(parts[0])
        for i in range(len(pwd)):
            tree['/'.join(pwd[:i+1])] += size

ans = 0
for size in tree.values():
    if size <= 100_000:
        ans += size

print(f'🎄 1. Solution: {ans}')

disc_space = 70_000_000
space_needed = 30_000_000
current_space = tree['/']
space_to_free = space_needed - (disc_space - current_space)

ans2 = min([size for size in tree.values() if size > space_to_free])
print(f'🎅 2. Solution: {ans2}')
