# https://adventofcode.com/2022/day/23

FILENAME = './2022/23/23.in'

raw = open(FILENAME).read()
lines = [line.strip() for line in open(FILENAME)]


elves = set()

for y, line in enumerate(lines):
    for x, item in enumerate(line):
        if item == '#':
            elves.add((x, y))

moves = [(0, -1), (0, 1), (-1, 0), (1, 0)]  # north, south, west, east
adjacent = [(-1, -1), (0, -1), (1, -1), (-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0)]

direction_map = {
    (0, -1): [(-1, -1), (0, -1), (1, -1)],
    (0, 1): [(-1, 1), (0, 1), (1, 1)],
    (-1, 0): [(-1, 0), (-1, 1), (-1, -1)],
    (1, 0): [(1, 0), (1, 1), (1, -1)]
}


def solve(elves, moves, turns):
    last_elves = set()
    iteration = 1

    for _ in range((turns if turns > 0 else 100_000)):
        once = set()
        twice = set()

        for elf in elves:
            if all((elf[0]+o[0], elf[1]+o[1]) not in elves for o in adjacent):
                continue
            for move in moves:
                if all((elf[0]+o[0], elf[1]+o[1]) not in elves for o in direction_map[move]):
                    new_position = (elf[0]+move[0], elf[1]+move[1])
                    if new_position in twice:
                        continue
                    elif new_position in once:
                        twice.add(new_position)
                    else:
                        once.add(new_position)
                    break

        elves_clone = set(elves)

        for elf in elves_clone:
            if all((elf[0]+o[0], elf[1]+o[1]) not in elves_clone for o in adjacent):
                continue
            for move in moves:
                if all((elf[0]+o[0], elf[1]+o[1]) not in elves_clone for o in direction_map[move]):
                    new_position = (elf[0]+move[0], elf[1]+move[1])
                    if new_position not in twice:
                        elves.remove(elf)
                        elves.add(new_position)
                    break

        moves.append(moves.pop(0))

        if last_elves == elves:
            return iteration

        last_elves = set(elves)
        iteration += 1

    mx = min(x for (x, y) in elves)
    Mx = max(x for (x, y) in elves)
    my = min(y for (x, y) in elves)
    My = max(y for (x, y) in elves)

    return int((Mx - mx + 1) * (My - my + 1)) - len(elves)


print(f'🎄 1. Solution: {solve(set(elves), list(moves), 10)}')
print(f'🎅 2. Solution: {solve(set(elves), list(moves), -1)}')
