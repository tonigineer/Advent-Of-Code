"""https://adventofcode.com/2020/day/1."""
import aocd
import itertools
import numpy as np


def part1(numbers, targetValue=2020):
    for number in numbers:
        for number2 in numbers:
            if number + number2 == targetValue:
                return number * number2


def part2(numbers, targetValue=2020):
    for number in numbers:
        for number2 in numbers:
            for number3 in numbers:
                if number + number2 + number3 == targetValue:
                    return number * number2 * number3


def refactored(numbers, depth, targetValue=2020):
    for values in itertools.combinations(numbers, depth):
        if sum(values) == targetValue:
            return np.prod(values)


if __name__ == "__main__":
    data = aocd.get_data(day=1, year=2020)
    numbers = list(map(int, data.splitlines()))
    print('Fast approach')
    print(f'Solution 1: {part1(numbers)}')
    print(f'Solution 2: {part2(numbers)}')

    print('\nOneliner')
    print(f'Solution 1: {[a*b for (a, b) in itertools.combinations(numbers, 2) if a+b == 2020][0]}')
    print(f'Solution 2: {[a*b*c for (a, b, c) in itertools.combinations(numbers, 3) if a+b+c == 2020][0]}')

    print('\nGeneric refactored')
    print(f'Solution 1: {refactored(numbers, 2)}')
    print(f'Solution 2: {refactored(numbers, 3)}')
