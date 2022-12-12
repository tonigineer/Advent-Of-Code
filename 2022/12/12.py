# https://adventofcode.com/2022/day/12

from collections import deque


lines = [line.strip() for line in open('./2022/12/12.in')]
G = [line for line in lines]

h = len(G)
w = len(G[0])

E = [[0 for _ in range(w)] for _ in range(h)]


def solve(start: str):
    Q = deque()

    for r in range(h):
        for c in range(w):
            E[r][c] = ord(G[r][c]) - ord('a')
            if G[r][c] == start:
                Q.append((c, r, 0))
                E[r][c] = 0
            if G[r][c] == 'E':
                E[r][c] = 26

    S = set()
    while Q:
        c, r, d = Q.popleft()

        if (c, r) in S:
            continue
        S.add((c, r))

        for dc, dr in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            cc = c + dc
            rr = r + dr
            if 0 <= cc < w and 0 <= rr < h:
                if E[rr][cc]-E[r][c] <= 1:
                    if G[rr][cc] == 'E':
                        return d + 1
                    Q.append((cc, rr, d + 1))


print(f'🎄 1. Solution: {solve("S")}')
print(f'🎅 2. Solution: {solve("a")}')
