""" https://adventofcode.com/2020/day/5 """

import aocd


def determineSeatId(bp):
    """Translate code into binary."""
    bp = bp.translate(str.maketrans('FBLR', '0101'))
    return calcSeatId(int(bp[:7], 2), int(bp[7:], 2))


def calcSeatId(row, col, factor=8):
    """Apply seat-id-formula."""
    return row * factor + col


def solvePuzzle(data, PART2):
    """Loop through all passports and determine ids."""
    seatIds = [determineSeatId(bp) for bp in data.splitlines()]
    if not PART2:
        return max(seatIds)
    return determineMySeat(seatIds)


def determineMySeat(seatIds):
    """Find free seat between occupied seats."""
    for seatId in range(min(seatIds), max(seatIds)):
        if (seatId not in seatIds) and (seatId+1 in seatIds) and (seatId-1 in seatIds):
            return seatId


if __name__ == "__main__":
    data = aocd.get_data(day=5, year=2020)
    print(f'Part 1: {solvePuzzle(data, PART2=False)}')
    print(f'Part 2: {solvePuzzle(data, PART2=True)}')
