""" https://adventofcode.com/2020/day/11 """

import aocd
from copy import deepcopy

class Game():
    occupied = '#'
    free = 'L'
    floor = '.'
    turn = 0

    def __init__(self, seats, part2):
        self.height = len(seats)
        self.width = len(seats[0])
        self.cells = dict()
        for y, row in enumerate(seats):
            for x, value in enumerate(row):
                self.cells[(x, y)] = value
        self.cellsLastRound = deepcopy(self.cells)
        self.part2 = part2


    def play(self):
        while True:
            self.turn += 1
            print(self.turn)
            self.cellsLastRound = deepcopy(self.cells)
            for position in self.cells.keys():
                adjacentSeatsOccupied = self._adjacentSeatsOccupied(position)
                if self.cells[position] == self.free and adjacentSeatsOccupied == 0:
                    self.cells[position] = self.occupied
                    continue
                if self.cells[position] == self.occupied and adjacentSeatsOccupied >= (4 if not self.part2 else 5):
                    self.cells[position] = self.free
                    continue
            if self.seatsAreStable():
                break
        return self.seatsTaken()

    def _adjacentSeatsOccupied(self, position):
        x, y = position[0], position[1]
        adjacentSeatsOccupied = 0
        if not self.part2:
            for x, y in [(x+i, y+j) for i in (-1,0,1) for j in (-1,0,1) if i != 0 or j != 0]:
                if (x, y) in self.cells:
                    if self.cellsLastRound[(x, y)] == self.occupied:
                        adjacentSeatsOccupied += 1
            return adjacentSeatsOccupied

        for dx in [-1, 0, 1]:
            for dy in [-1, 0, 1]:
                if dx == 0 and dy == 0:
                    continue
                x, y = position[0], position[1]
                while True:
                    x = x + dx
                    y = y + dy
                    if (x, y) in self.cells:
                        if self.cellsLastRound[(x, y)] == self.free:
                            break
                        if self.cellsLastRound[(x, y)] == self.occupied:
                            adjacentSeatsOccupied += 1
                            break
                    else:
                        break
        return adjacentSeatsOccupied

    def seatsAreStable(self):
        for cell in self.cells.keys():
            if self.cells[cell] != self.cellsLastRound[cell]:
                return False
        return True

    def seatsTaken(self):
        taken = 0
        for cell in self.cells.keys():
            if self.cells[cell] == self.occupied:
                taken += 1
        return taken


if __name__ == "__main__":
    seats = aocd.get_data(day=11, year=2020).splitlines()
    game = Game(seats, False)
    print(f'Part 1: {game.play()}')
    game = Game(seats, True)
    print(f'Part 2: {game.play()}')