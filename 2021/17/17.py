"""https://adventofcode.com/2021/day/17."""

import re


with open('data/17.in') as f:
    lines = f.read()

x_s, x_e, y_e, y_s = list(map(int, re.findall('[0-9]+', lines)))
y_s, y_e = -y_s, -y_e

y_max = 0
hits = 0

educated_guess = 300

for dx_s in range(0, educated_guess):
    for dy_s in range(-educated_guess, educated_guess):
        dx, dy = dx_s, dy_s
        y_max_temp = 0
        x, y = 0, 0

        while x <= x_e and y >= y_e:
            x += dx
            y += dy
            y_max_temp = max(y, y_max_temp)

            if x_s <= x <= x_e and y_e <= y <= y_s:
                hits += 1
                y_max = max(y_max, y_max_temp)
                break

            if dx > 0: dx -= 1
            dy -= 1

print(f'Max y position: {y_max}')
print(f'Hits: {hits}')
