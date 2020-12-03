""" https://adventofcode.com/2020/day/2 """

import aocd
import re

pattern = re.compile(r'^(?P<min_>\d+)-(?P<max_>\d+) (?P<letter>[a-z]):\s*(?P<password>[a-z]+)$')


def solvePart1(passwords):
    validPasswords = 0
    for password in passwords:
        m = pattern.search(password)
        min_ = int(m['min_'])
        max_ = int(m['max_'])
        num = m['password'].count(m['letter'])
        if min_ <= num <= max_:
            validPasswords += 1
    return validPasswords


def solvePart2(passwords):
    validPasswords = 0
    for password in passwords:
        m = pattern.search(password)
        pos1 = int(m['min_']) - 1
        pos2 = int(m['max_']) - 1
        pass_ = m['password']
        letter = m['letter']
        if (pass_[pos1] == letter) ^ (pass_[pos2] == letter):
            validPasswords += 1
    return validPasswords


if __name__ == "__main__":
    data = aocd.get_data(day=2, year=2020)
    passwords = data.splitlines()
    print(f'Part 1: {solvePart1(passwords)}')
    print(f'Part 2: {solvePart2(passwords)}')
