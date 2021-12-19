"""https://adventofcode.com/2021/day/16."""


with open('data/16.in') as f:
    lines = f.read()

binary = bin(int(lines, 16))[2:]
binary = '0' * (len(binary) % 4) + binary


def parse(binary):
    version = binary[0:3]
    type_ = binary[3:6]
    i = 6

    if int(type_, 2) != 4:
        i += 1
        length_id = binary[i]
        n = 11 if length_id == '1' else 15
        operator = binary[i:i+n]
        i += n

    # literal values
    literal_value = ''
    while True:
        literal_value += binary[i+1:i+5]
        if binary[i] != '1':
            break
        i += 5

    print(int(literal_value, 2))
    return int(literal_value, 2)

1 1010 0 0101 0

parse(binary)