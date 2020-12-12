""" https://adventofcode.com/2020/day/12 """

import aocd

headings = {0: 'N', 90: 'E', 180: 'S', 270: 'W'}


class Navigation():
        def __init__(self, instructions, x0=0, y0=0, heading0=90):
            self.x = x0
            self.y = y0
            self.xShip = 0
            self.yShip = 0
            self.heading = heading0
            self.instructions = instructions

        def navigate(self, cmd):
            if cmd[0] == 'F':
                cmd = f'{headings[self.heading]}{cmd[1:]}'
            if cmd[0] == 'N':
                self.y = self.y + int(cmd[1:])
            if cmd[0] == 'S':
                self.y = self.y - int(cmd[1:])
            if cmd[0] == 'W':
                self.x = self.x - int(cmd[1:])
            if cmd[0] == 'E':
                self.x = self.x + int(cmd[1:])
            if cmd[0] == 'R':
                self.heading = (self.heading + int(cmd[1:])) % 360
                if self.part2:
                    for i in range(int(int(cmd[1:])/90)):
                        x = self.y
                        y = -self.x
                        self.x, self.y = x, y
            if cmd[0] == 'L':
                self.heading = (self.heading - int(cmd[1:])) % 360
                if self.part2:
                    for i in range(int(int(cmd[1:])/90)):
                        x = -self.y
                        y = self.x
                        self.x, self.y = x, y

        def move(self, repetitions):
            self.xShip += self.x*int(repetitions)
            self.yShip += self.y*int(repetitions)

        def start(self, part2):
            self.part2 = part2
            for cmd in self. instructions:
                if part2:
                    if cmd[0] == 'F':
                        self.move(cmd[1:])
                        continue
                self.navigate(cmd)

            if not part2:
                return abs(self.x) + abs(self.y)
            return abs(self.xShip) + abs(self.yShip)


if __name__ == "__main__":
    data = aocd.get_data(day=12, year=2020).splitlines()
    print(f'Part 1: {Navigation(data).start(False)}')
    print(f'Part 2: {Navigation(data, 10, 1).start(True)}')
