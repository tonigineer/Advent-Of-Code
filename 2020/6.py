""" https://adventofcode.com/2020/day/6 """

import aocd


def solvePuzzle(data, PART2):
    numOfAnswers = 0
    numEverybodyAnswered = 0
    for group in data.split('\n\n'):
        answers = ''.join(group.replace('\n', ''))
        numOfAnswers += len(set(answers))

        for answer in set(answers):
            if answers.count(answer) == len(group.split('\n')):
                numEverybodyAnswered += 1

    if not PART2:
        return numOfAnswers
    return numEverybodyAnswered


if __name__ == "__main__":
    data = aocd.get_data(day=6, year=2020)
    print(f'Part 1: {solvePuzzle(data, PART2=False)}')
    print(f'Part 2: {solvePuzzle(data, PART2=True)}')