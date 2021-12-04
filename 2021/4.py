"""https://adventofcode.com/2021/day/4."""


class Board():
    """Class to handle bingo board."""

    BOARD_SIZE = 5

    def __init__(self):
        """No summary line needed ;)."""
        self.rows = list()
        self.columns = list()
        self.finished = False

    def add_row(self, row):
        """Append row to board.

        When all rows are filled, columns for bingo check
        are created.
        """
        self.rows.append(row)
        if len(self.rows) >= self.BOARD_SIZE:
            self._create_columns()

    def _create_columns(self):
        """Create columns from rows to have an easier check for bingo later."""
        for i in range(0, self.BOARD_SIZE):
            self.columns.append([_[i] for _ in self.rows])

    def mark_number(self, number):
        """Mark drawn bingo number and check for bingo."""
        for i, _ in enumerate(self.rows):
            if number in _:
                self.rows[i].remove(number)

        for i, _ in enumerate(self.columns):
            if number in _:
                self.columns[i].remove(number)

        return self.check_bingo()

    def check_bingo(self):
        """Check for bingo."""
        for _ in self.rows:
            if len(_) == 0:
                return True
        for _ in self.columns:
            if len(_) == 0:
                return True
        return False


def part1(boards, numbers_drawn):
    """Solve first part of puzzle.

    Find first board with bingo.
    """
    for number in numbers_drawn:
        for board in boards:
            if board.mark_number(number):
                for _ in board.rows:
                    sumsum = sum([sum(_) for _ in board.rows])
                    print(f'Part 1: {sumsum * number}')
                    return


def part2(boards, numbers_drawn):
    """Solve second part of puzzle.

    Find last board with bingo.
    """
    boards_finished = 0
    for number in numbers_drawn:
        for board in boards:
            if board.mark_number(number):
                for _ in board.rows:
                    sumsum = sum([sum(_) for _ in board.rows])
                    if board.finished is False:
                        board.finished = True
                        boards_finished += 1
                        if boards_finished == len(boards):
                            print(f'Part 2: {sumsum * number}')
                            return


if __name__ == "__main__":
    with open('data/4.in') as f:
        data = f.read().splitlines()

    numbers_drawn = list(map(int, data.pop(0).split(',')))
    data = [_ for _ in data if len(_) > 0]

    boards = list()
    for idx, row in enumerate(data):
        if (idx % Board().BOARD_SIZE) == 0:
            boards.append(Board())
        row_data = list(map(int, row.split()))
        boards[-1].add_row(row_data)

    part1(boards, numbers_drawn)
    part2(boards, numbers_drawn)
