# https://adventofcode.com/2022/day/2

lines = [line.strip() for line in open('./2022/inputs/02.in')]

score = 0

for line in lines:
    opp, me = line.split()
    score += {'X': 1, 'Y': 2, 'Z': 3}[me]
    score += {('A', 'Z'): 0, ('B', 'X'): 0, ('C', 'Y'): 0,
              ('A', 'X'): 3, ('B', 'Y'): 3, ('C', 'Z'): 3,
              ('C', 'X'): 6, ('A', 'Y'): 6, ('B', 'Z'): 6}[(opp, me)]

print(f'1. Solution: {score}')

score = 0

for line in lines:
    opp, res = line.split()
    score += {'X': 0, 'Y': 3, 'Z': 6}[res]
    score += {('A', 'Z'): 2, ('B', 'X'): 1, ('C', 'Y'): 3,
              ('A', 'X'): 3, ('B', 'Y'): 2, ('C', 'Z'): 1,
              ('C', 'X'): 2, ('A', 'Y'): 1, ('B', 'Z'): 3}[(opp, res)]

print(f'2. Solution: {score}')
