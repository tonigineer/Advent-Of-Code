""" https://adventofcode.com/2020/day/8 """

import aocd


class Booting():
    """Compute boot instructions."""

    def __init__(self, bootCode):
        """Initialize booting."""
        self.accumulator = 0
        self.index = 0
        self.indiciesExecuted = []
        self.terminate = False
        self.successful = False
        self.bootCode = bootCode
        self.numberOfInstructions = len(bootCode)

    def acc(self, arg):
        """ACC instruction."""
        self.accumulator += arg
        self.index += 1

    def jmp(self, arg):
        """JMP instruction."""
        self.index += arg

    def nop(self, arg):
        """NOP instruction."""
        self.index += 1

    def executeIntruction(self):
        """Execute instructions."""
        fcn, arg = self.bootCode[self.index].split(' ')
        arg = int(arg)
        self.indiciesExecuted.append(self.index)

        if fcn == 'acc':
            self.acc(arg)
        if fcn == 'jmp':
            self.jmp(arg)
        if fcn == 'nop':
            self.nop(arg)

    def checkIfTermination(self):
        """Terminate if conditions are fullfiled."""
        if self.index in self.indiciesExecuted:
            self.terminate = True
            return

        if self.index >= self.numberOfInstructions:
            self.successful = True
            self.terminate = True

    def runBootCode(self):
        """Run program til termination."""
        while not self.terminate:
            self.executeIntruction()
        return self.accumulator, self.successful


def repairBootCode(bootCode):
    """Loop over bootCode and try fixing error by swapping commands (only one)."""
    for index in range(len(bootCode)):
        boot = Booting(aocd.get_data(day=8, year=2020).splitlines())
        instruction = boot.bootCode[index].split(' ')[0]

        if instruction == 'acc':
            continue

        if instruction == 'nop':
            boot.bootCode[index] = boot.bootCode[index].replace('nop', 'jmp')
        else:
            boot.bootCode[index] = boot.bootCode[index].replace('jmp', 'nop')

        _, success = boot.runBootCode()

        if success:
            return boot.accumulator


if __name__ == "__main__":
    bootCode = aocd.get_data(day=8, year=2020).splitlines()
    print(f'Part 1: {Booting(bootCode).runBootCode()[0]}')
    print(f'Part 2: {repairBootCode(bootCode)}')
