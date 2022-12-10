# https://adventofcode.com/2022/day/10

lines = [line.strip() for line in open('./2022/10/10.in')]

w, h = 40, 6
crt = [['?' for _ in range(w)] for _ in range(h)]

ans, x, t = 0, 1, 0


def cycle(t, ans):
    t += 1
    if t in [20, 60, 100, 140, 180, 220]:
        ans += t * x

    crt[(t-1)//w][(t-1) % w] = ('#' if abs(x - ((t-1) % w)) <= 1 else ' ')

    return t, ans


for cmd in lines:
    if cmd == 'noop':
        t, ans = cycle(t, ans)
    else:
        _, v = cmd.split()
        t, ans = cycle(t, ans)
        t, ans = cycle(t, ans)
        x += int(v)


print(f'🎄 1. Solution: {ans}')
print(f'🎅 2. Solution')
for r in crt:
    print(''.join(r))
