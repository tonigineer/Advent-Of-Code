# https://adventofcode.com/2022/day/5

from copy import deepcopy


lines = [line.strip() for line in open('./2022/05/05.in')]

stacks = [
    'DBJV',
    'PVBWRDF',
    'RGFLDCWQ',
    'WJPMLNDB',
    'HNBPCSQ',
    'RDBSNG',
    'ZBPMQFSH',
    'WLF',
    'SVFMR'
]

stacks_orig = deepcopy(stacks)
lines = lines[10:]

for cmd in lines:
    cmd = cmd.split()
    num, from_, to_ = int(cmd[1]), int(cmd[3])-1, int(cmd[5])-1
    stacks[to_] += stacks[from_][-num:][::-1]
    stacks[from_] = stacks[from_][:-num]

print(f'🎄 1. Solution: {"".join([s[-1] for s in stacks if len(s) > 0])}')

stacks = stacks_orig
for cmd in lines:
    cmd = cmd.split()
    num, from_, to_ = int(cmd[1]), int(cmd[3])-1, int(cmd[5])-1
    stacks[to_] += stacks[from_][-num:]#[::-1]
    stacks[from_] = stacks[from_][:-num]

print(f'🎅 2. Solution: {"".join([s[-1] for s in stacks if len(s) > 0])}')
