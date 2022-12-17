# https://adventofcode.com/2022/day/16

from copy import deepcopy
from collections import defaultdict

FILENAME = './2022/16/16.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

C = defaultdict(list)
R = defaultdict(int)

for line in lines:
    parts = line.split()
    _id = parts[1]
    rate = int(parts[4][5:-1])
    con = [c for c in "".join(parts[9:]).split(',')]
    for c in con:
        C[_id].append(c)
    R[_id] = rate


S = {}


def v(_id, _open, _time):
    if _time == 0:
        return 0

    key = (_id, tuple(sorted(_open)), _time)
    if key in S:
        return S[key]

    ans = 0

    if _time > 0:
        if _id not in _open and R[_id] > 0:
            _open_new = deepcopy(_open)
            _open_new.add(_id)
            ans = max(
                ans,
                sum(R[_id] for _id in _open) + v(_id, _open_new, _time-1)
            )

        for c in C[_id]:
            ans = max(
                ans,
                sum(R[_id] for _id in _open) + v(c, _open, _time-1)
            )

    S[key] = ans
    return ans


print(f'🎄 1. Solution: {v("AA", set(), 30)}')

# while Q:
#     score, _id, _open, _time = Q.popleft()
#     print(_time)
#     ans = max(ans, score)

#     key = (_id, tuple(sorted(_open)), _time)
#     if key in S:
#         # if S[key] > score:
#         continue

#     if _time > 0:
#         score = score + sum([R[_id] for _id in _open])

#         for c in C[_id]:
#             S[(_id, tuple(sorted(_open)), _time)] = score
#             Q.append([score, c, _open, _time - 1])

#         if _id not in _open and R[_id] > 0:
#             _open = deepcopy(_open)
#             _open.add(_id)
#             S[(_id, tuple(sorted(_open)), _time)] = score
#             Q.append([score, _id, _open, _time - 1])



# print(ans)

# print(f'🎅 2. Solution: {xn * MAX_COORD + yn}')
