# https://adventofcode.com/2022/day/17

from collections import deque


FILENAME = './2022/18/18.in'

raw = open(FILENAME).read().strip()
lines = [line.strip() for line in open(FILENAME)]

offsets = [(0.5, 0, 0), (0, 0.5, 0), (0, 0, 0.5), (-0.5, 0, 0), (0, -0.5, 0), (0, 0, -0.5)]

cubes = set()
faces = {}

for line in lines:
    x, y, z = map(int, line.split(','))
    cubes.add((x, y, z))

    for dx, dy, dz in offsets:
        face = (x+dx, y+dy, z+dz)
        if face not in faces:
            faces[face] = 0
        faces[face] += 1

print(f'🎄 1. Solution: {list(faces.values()).count(1)}')


def _neighbors(point):
    x, y, z = point
    return [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]


min_x, max_x = min(x for x, _, _ in cubes), max(x for x, _, _ in cubes)
min_y, max_y = min(y for _, y, _ in cubes), max(y for _, y, _ in cubes)
min_z, max_z = min(z for _, _, z in cubes), max(z for _, _, z in cubes)
outside = {(min_x - 1, min_y - 1, min_z - 1)}

queue = deque(outside)
while queue:
    for point in _neighbors(queue.popleft()):
        if point in cubes or point in outside:
            continue

        x, y, z = point
        if not (min_x - 1 <= x <= max_x + 1 and min_y - 1 <= y <= max_y + 1 and min_z - 1 <= z <= max_z + 1):
            continue

        outside.add(point)
        queue.append(point)


print(f'🎅 2. Solution: {sum(neighbor in outside for cube in cubes for neighbor in _neighbors(cube))}')
