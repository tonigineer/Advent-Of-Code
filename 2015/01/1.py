"""https://adventofcode.com/2015/day/1."""
import aocd


def part1(data):
    """Simply count up and downs."""
    floor = 0
    for char in data:
        if char == "(":
            floor += 1
        elif char == ")":
            floor -= 1
        else:
            raise ValueError('alles kaputt oO')
    return floor


def part2(data):
    """Simply count up and downs and quit when in the basement."""
    floor = 0
    targetFloor = -1
    for idx, char in enumerate(data):
        if char == "(":
            floor += 1
        elif char == ")":
            floor -= 1
        else:
            raise ValueError('alles kaputt oO')

        if floor <= targetFloor:
            break
    return idx + 1


if __name__ == "__main__":
    data = aocd.get_data(day=1, year=2015)
    print(f'Solution 1: {part1(data)}')
    print(f'Solution 2: {part2(data)}')
