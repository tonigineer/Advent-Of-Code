# https://adventofcode.com/2022/day/21

FILENAME = './2022/21/21.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

monkeys = {}

for line in lines:
    monkey, expr = line.split(': ')
    if expr.isdigit():
        monkeys[monkey] = int(expr)
    else:
        left, op, right = expr.split(' ')
        if left in monkeys and right in monkeys:
            monkeys[monkey] = eval(f'{monkeys[left]} {op} {monkeys[right]}')
        else:
            lines.append(line)

print(f'🎄 1. Solution: {int(monkeys["root"])}')


def solve(lines, humn):
    monkeys = {'humn': humn}

    for line in lines:
        monkey, expr = line.split(': ')
        if monkey in monkeys:
            continue
        if expr.isdigit():
            monkeys[monkey] = int(expr)
        else:
            left, op, right = expr.split(' ')
            if left in monkeys and right in monkeys:
                if monkey == 'root':
                    return monkeys[left] - monkeys[right]
                monkeys[monkey] = eval(f'{monkeys[left]} {op} {monkeys[right]}')
            else:
                lines.append(line)


lines = [line.strip() for line in open(FILENAME)]

low = 0
high = 1e15

while low < high:
    mid = (low + high) // 2
    val = solve(lines, mid)
    if abs(val) < 0.5:
        break
    elif val > 0:
        # val < 0 for example, therefore might not work for all puzzle!
        low = mid
    else:
        high = mid

print(f'🎅 2. Solution: {int(mid)}')
