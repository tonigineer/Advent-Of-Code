"""https://adventofcode.com/2021/day/14."""


with open('data/14.in') as f:
    lines = f.read().splitlines()

template = lines[0]

pairs = dict()
for x, y in zip(template[:-1], template[1:]):
    if x + y not in pairs.keys():
        pairs[x + y] = 0
    pairs[x+y] += 1

commands = {p: c for p, c in [_.split(' -> ') for _ in lines[2:]]}

for _ in range(40):
    npairs = {}
    for pair in pairs:
        command = commands[pair]
        p1 = pair[0]+command
        p2 = command+pair[1]
        if p1 not in npairs:
            npairs[p1] = 0
        npairs[p1] += pairs[pair]
        if p2 not in npairs:
            npairs[p2] = 0
        npairs[p2] += pairs[pair]
    pairs = npairs

letters = dict()
for pair, amount in pairs.items():
    x, y = pair
    if x not in letters:
        letters[x] = 0
    letters[x] += amount
letters[y] += 1

print(max(letters.values())-min(letters.values()))
