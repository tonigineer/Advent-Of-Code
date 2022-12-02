""" https://adventofcode.com/2020/day/4 """

import aocd
import re

REQUIREMENTS = [
    ['byr', lambda x: 1920 <= int(x) <= 2002],
    ['iyr', lambda x: 2010 <= int(x) <= 2020],
    ['eyr', lambda x: 2020 <= int(x) <= 2030],
    ['hgt', lambda x: (x[-2:] == 'cm' and 150 <= int(x[:-2]) <= 193) or
                      (x[-2:] == 'in' and 59 <= int(x[:-2]) <= 76)],
    ['hcl', lambda x: re.fullmatch(r'#[0-9a-f]{6}', x)],
    ['ecl', lambda x: x in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth')],
    ['pid', lambda x: re.fullmatch(r'[0-9]{9}', x)]
]


def loopThroughPassports(data, PART2):
    """Extract passports from data and check validation."""
    validPassports = 0
    for passportData in data.split('\n\n'):
        passport = dict()
        for line in passportData.splitlines():
            for entry in line.split(' '):
                key, value = entry.split(':')
                passport[key] = value
        validPassports += isValid(passport, PART2)
    return validPassports


def isValid(passport, PART2):
    """Check if requirements are given and fullfilled."""
    for key, req in REQUIREMENTS:
        if key not in passport.keys():
            return 0
        if PART2:
            if not req(passport[key]):
                return 0
    return 1


if __name__ == "__main__":
    data = aocd.get_data(day=4, year=2020)
    print(f'Part 1: {loopThroughPassports(data, PART2=False)}')
    print(f'Part 2: {loopThroughPassports(data, PART2=True)}')
