# https://adventofcode.com/2022/day/19

import re

FILENAME = './2022/19/19.in'
RES = ['ore', 'clay', 'obsidian', 'geode']

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]


def dfs(bp, max_spend, cache, time, bots, amts):
    # ['ore', 'clay', 'obsidian', 'geode']
    if time == 0:
        return amts[3]

    key = tuple([time, *bots, *amts])
    if key in cache:
        return cache[key]

    max_score = 0
    max_score = amts[3] + bots[3] * time  # doing nothing

    for _type, recipe in enumerate(blueprint):
        if _type != 3 and bots[_type] >= max_spend[_type]:
            continue

        # how long to wait to build a new bot
        wait = 0
        for res, amt in recipe:
            if bots[res] == 0:
                break
            wait = max(wait, -(-(amt - amts[res]) // bots[res]))
        else:
            # determine state when bot build
            time_remain = time - wait - 1
            if time_remain <= 0:
                continue
            bots_ = bots[:]
            amts_ = [amt + bot * (wait + 1) for bot, amt in zip(bots, amts)]

            for res, amt in recipe:
                amts_[res] -= amt
            bots_[_type] += 1
            for i in range(3):
                amts_[i] = min(amts_[i], max_spend[i] * time_remain)

            max_score = max(max_score, dfs(blueprint, max_spend, cache, time_remain, bots_, amts_))

    cache[key] = max_score
    return max_score


part1 = 0
part2 = 1

for idx, line in enumerate(lines):
    blueprint = []
    max_spend = [0, 0, 0]

    for section in line.split(':')[1].strip().split('.'):
        recipe = []
        for amt, res in re.findall(r"(\d+) (\w+)", section):
            amt = int(amt)
            res = RES.index(res)
            recipe.append((res, amt))
            max_spend[res] = max(max_spend[res], amt)
        if recipe:
            blueprint.append(recipe)

    score = dfs(blueprint, max_spend, {}, 24, [1, 0, 0, 0], [0, 0, 0, 0])
    part1 += score * (idx+1)

    if idx <= 2:
        score = dfs(blueprint, max_spend, {}, 32, [1, 0, 0, 0], [0, 0, 0, 0])
        part2 *= score


print(f'🎄 1. Solution: {part1}')
print(f'🎅 2. Solution: {part2}')
