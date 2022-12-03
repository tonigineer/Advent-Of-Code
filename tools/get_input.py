import sys

from os import path, mkdir
from datetime import date

from aocd import get_data


def main():
    if len(sys.argv) == 1:
        if date.today().month != 12:
            raise ValueError(
                'This is no AoC at this time of the year. '
                'Please enter a day and a year as command line argument. '
                'e.g.: python get_input.py 1 2016'
            )
        day = date.today().day
        year = date.today().year
    else:
        day, year = map(int, sys.argv[1:3])

    input_path = f'{year}/{str(day).zfill(2)}'
    if not path.exists(input_path):
        mkdir(input_path)

    full_file_name = f'{input_path}/{str(day).zfill(2)}.in'
    if path.exists(full_file_name):
        raise FileExistsError(
            f'Input file for {year}/{str(day).zfill(2)} already exists.'
        )

    input_data = get_data(day=day, year=year).split('\n')

    with open(full_file_name, 'w') as f:
        f.write('\n'.join(input_data))

    print(f'{full_file_name} was created.')


if __name__ == "__main__":
    main()
