# https://adventofcode.com/2022/day/10

data = open('./2022/11/11.in').read()


def solve(part):
    M = []
    IT = []
    OP = []
    TEST = []
    TRUE = []
    FALSE = []

    for m in data.split('\n\n'):
        id_, it, op, test, true, false = m.split('\n')

        IT.append([int(i) for i in it.split(':')[1].split(',')])

        op = ''.join(op.split()[-3:])
        OP.append(lambda old, op=op: eval(op))

        TEST.append(int(test.split()[3]))

        TRUE.append(int(true.split()[-1]))
        FALSE.append(int(false.split()[-1]))

    if part == 10000:
        lcm = 1
        for div in TEST:
            lcm *= (div*lcm)

    INS = [0 for _ in range(len(OP))]
    for _ in range(part):
        for m in range(len(OP)):
            for it in IT[m]:
                INS[m] += 1
                it = OP[m](it)

                if part == 20:
                    it = (it // 3)
                else:
                    it = it % lcm

                if it % TEST[m] == 0:
                    IT[TRUE[m]].append(it)
                else:
                    IT[FALSE[m]].append(it)
            IT[m] = []

    INS = sorted(INS)
    return INS[-1] * INS[-2]


print(f'🎄 1. Solution: {solve(20)}')
print(f'🎅 2. Solution:  {solve(10000)}')
