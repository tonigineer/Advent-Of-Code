# https://adventofcode.com/2022/day/6


line = [line.strip() for line in open('./2022/06/06.in')][0]


d = 4
for i in range(d, len(line)):
    if len(set([line[i-j] for j in range(d)])) == d:
        print(f'🎄 1. Solution: {i+1}')
        break


d = 14
for i in range(d, len(line)):
    if len(set([line[i-j] for j in range(d)])) == d:
        print(f'🎅 2. Solution: {i+1}')
        break
