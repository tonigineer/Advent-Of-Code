# https://adventofcode.com/2022/day/16


from collections import deque
from collections import defaultdict


FILENAME = './2022/16/16.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

tunnels = defaultdict(list)
rates = defaultdict(int)

for line in lines:
    parts = line.split()
    valve = parts[1]

    rate = int(parts[4][5:-1])
    rates[valve] = rate

    connections = [c for c in "".join(parts[9:]).split(',')]
    for c in connections:
        tunnels[valve].append(c)


dists = {}  # get rid of valves without pressure rates and get distance to them
for valve in rates.keys():
    if valve != 'AA' and rates[valve] == 0:
        continue

    dists[valve] = {valve: 0, 'AA': 0}
    seen = {valve}

    queue = deque([(0, valve)])

    while queue:
        distance, position = queue.popleft()
        for neighbor in tunnels[position]:
            if neighbor in seen:
                continue
            seen.add(neighbor)
            if rates[neighbor]:
                dists[valve][neighbor] = distance + 1
            queue.append((distance + 1, neighbor))

    del dists[valve][valve]
    if valve != 'AA':
        del dists[valve]['AA']


cache = {}
indices = {valve: idx for idx, valve in enumerate(dists.keys())}


def dfs(time, valve, bit_mask):
    if time == 0:
        return 0

    if (time, valve, bit_mask) in cache:
        return cache[(time, valve, bit_mask)]

    score = 0

    if time > 0:
        bit = 1 << indices[valve]

        if not (bit_mask & bit) and rates[valve] > 0:
            score = max(score, rates[valve]*(time - 1) + dfs(time-1, valve, bit_mask | bit))

        for neighbor, distance in dists[valve].items():
            score = max(score, dfs(time-distance, neighbor, bit_mask))

    cache[(time, valve, bit_mask)] = score
    return score


print(f'🎄 1. Solution: {dfs(30, "AA", 0)}')


wtf = (1 << len(dists)) - 1
max_score = 0

for i in range((wtf + 1) // 2):
    max_score = max(max_score, dfs(26, "AA", i) + dfs(26, "AA", wtf ^ i))
    print(f'\r🎅 2. Solution: {max_score} ... ({i / (wtf // 2) * 100:0.1f}%)', end='')
print(f'\r🎅 2. Solution: {max_score}')
