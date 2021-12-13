"""https://adventofcode.com/2021/day/12."""


def solve1(current, itinerary):
    """Solution for first puzzle.

    Simply a recursive solution.
    """
    if current == 'end':
        solutions.append(itinerary.append('end'))
        return
    for _in, _out in connections:
        if _in == current:
            if _in.islower() and _in in itinerary:
                continue
            new = [_ for _ in itinerary]
            new.append(_in)
            solve1(_out, new)


def solve2(current, itinerary):
    """Solution for second puzzle.

    Same as first puzzle, but with additional
    conditions.
    """
    if current == 'end':
        solutions.append(itinerary.append('end'))
        return
    for _in, _out in connections:
        if _in == current:
            if _in.islower() and _in in itinerary:
                if _in == 'start' or _in == 'end':
                    continue
                lower_cases = [_ for _ in itinerary if _.islower()]
                if len(lower_cases) > len(set(lower_cases)):
                    continue
            new = [_ for _ in itinerary]
            new.append(_in)
            solve2(_out, new)


with open('data/12.in') as f:
    data = f.read().splitlines()
    connections = list()
    for _in, _out in [line.split('-') for line in data]:
        connections.append((_in, _out))
        connections.append((_out, _in))

solutions = list()
solve1('start', [])
print(f'Part 1: {len(solutions)}')

solutions = list()
solve2('start', [])
print(f'Part 2: {len(solutions)}')
