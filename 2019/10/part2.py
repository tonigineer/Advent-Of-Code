from math import *
def compute_lines(field, y0, x0):
    lines = {}
    for y in range(len(field)):
        for x in range(len(field[0])):
            if field[y][x] == "#":
                tmp = atan2((x-x0),(y-y0)) - pi/2
                if tmp in lines:
                    lines[tmp].append((y,x))
                    lines[tmp].sort(key=lambda x: abs(x[0]-x0)+abs(x[1]-y0))
                else: lines[tmp] = [(y,x)]
    return len(lines),(y0,x0),sorted(list(zip(lines.keys(), lines.values())),key = lambda x: x[0], reverse=True)
with open("asteroid_map.txt") as file:
    field = [list(line.replace("\n","")) for line in file]
    max_ast = max([compute_lines(field, y, x) if field[y][x] == "#" else (0,0) for y in range(len(field)) for x in range(len(field[0]))], key=lambda x:x[0])
    idx = [x[0] for x in max_ast[2]].index(pi/2)
    print(max_ast[0])
    for i in range(1,201):
        print(f"deleted point {i} is {max_ast[2][idx][1][0][::-1]}") #if i == 200 else None
        max_ast[2][idx] = max_ast[2][idx][1:]
        idx = (idx + 1) % len(max_ast[2])