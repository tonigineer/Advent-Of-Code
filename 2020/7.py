""" https://adventofcode.com/2020/day/7 """

import aocd
import re

patternBag = re.compile(r'([a-z ]+) bags contain')
patternContent = re.compile(r'(\d+) ([a-z ]+) bag')


def extractBags(data):
    """Create dict with list of contained bags."""
    return {patternBag.findall(bag)[0]: patternContent.findall(bag) for bag in data.split('\n')}


def bagInBag(allBags, currentBag, targetBag='shiny gold'):
    """Recursive search for target bag."""
    for bag in allBags[currentBag]:
        if bag[1] == targetBag or bagInBag(allBags, bag[1]):
            return True
    return False


def contentOfBag(allBags, rootBag='shiny gold'):
    """Count bags within bag."""
    return sum(int(bag[0]) * contentOfBag(allBags, bag[1]) for bag in allBags[rootBag]) + 1  # root bag


def solvePuzzle(data, PART2):
    """Solve puzzles."""
    bags = extractBags(data)
    if not PART2:
        return sum([bagInBag(bags, bag) for bag in bags.keys()])
    return contentOfBag(bags) - 1  # first root bag does not count


if __name__ == "__main__":
    data = aocd.get_data(day=7, year=2020)
    print(f'Part 1: {solvePuzzle(data, PART2=False)}')
    print(f'Part 2: {solvePuzzle(data, PART2=True)}')
