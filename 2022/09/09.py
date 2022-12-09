# https://adventofcode.com/2022/day/9


lines = [line.strip() for line in open('./2022/09/09.in')]


def keep_track(h, t):
    dx = t[0] - h[0]
    dy = t[1] - h[1]
    if abs(dx) == 2 or abs(dy) == 2:
        t[0] = h[0] + int(dx / 2)
        t[1] = h[1] + int(dy / 2)


def solve(knots):
    T = [[0, 0] for _ in range(knots)]

    visited = set()

    for line in lines:
        _dir, n = line.split()
        c, r = {'U': (0, 1), 'D': (0, -1), 'L': (-1, 0), 'R': (1, 0), }[_dir]

        for _ in range(int(n)):
            T[0][0] += c
            T[0][1] += r
            for i in range(1, knots):
                keep_track(T[i-1], T[i])

            visited.add((T[-1][0], T[-1][1]))
    return visited


print(f'🎄 1. Solution: {len(solve(2))}')
print(f'🎅 2. Solution: {len(solve(10))}')
