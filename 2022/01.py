# https://adventofcode.com/2022/day/1

import re

with open('./2022/inputs/1.in') as f:
    lines = f.read()

blank_line_regex = r"(?:\r?\n){2,}"

elves = re.split(blank_line_regex, lines.strip())
total_rations = [sum(map(int, elv.split('\n'))) for elv in elves]

print(f'1. Solution: {max(total_rations)}')
print(f'2. Solution: {sum(sorted(total_rations)[-3:])}')
