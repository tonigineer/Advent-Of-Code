use common::{Answer, Solution};

use std::cmp;

pub struct Day02;

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Bathroom Security"
    }

    fn part1(&self, input: &str) -> Answer {
        return part1(&input.to_string()).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return part2(&input.to_string()).into();
    }
}


fn part1(input: &String) -> String {
    const KEYPAD: [[char; 3]; 3] = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9']
    ];

    let mut position: (i32, i32) = (1, 1);
    let mut code: String = String::from("");

    for line in input.lines() {
        for char in line.chars() {
            position = match char {
                'L' => (cmp::max(position.0 - 1, 0), position.1),
                'R' => (cmp::min(position.0 + 1, 2), position.1),
                'U' => (position.0, cmp::max(position.1 - 1, 0)),
                'D' => (position.0, cmp::min(position.1 + 1, 2)),
                _ => panic!("Direction not defined"),
            };
        }

        code = format!(
            "{}{}",
            code,
            KEYPAD[position.1 as usize][position.0 as usize]
        )
    }

    return code;
}

fn part2(input: &String) -> String {
    const KEYPAD: [[char; 5]; 5] = [
        ['.', '.', '1', '.', '.'],
        ['.', '2', '3', '4', '.'],
        ['5', '6', '7', '8', '9'],
        ['.', 'A', 'B', 'C', '.'],
        ['.', '.', 'D', '.', '.'],
    ];

    let mut position: (i32, i32) = (1, 1);
    let mut code: String = String::from("");

    for line in input.lines() {
        for char in line.chars() {
            let new_position = match char {
                'L' => (cmp::max(position.0 - 1, 0), position.1),
                'R' => (cmp::min(position.0 + 1, 4), position.1),
                'U' => (position.0, cmp::max(position.1 - 1, 0)),
                'D' => (position.0, cmp::min(position.1 + 1, 4)),
                _ => panic!("Direction not defined"),
            };

            if !(KEYPAD[new_position.1 as usize][new_position.0 as usize] == '.') {
                position = new_position
            }
         }

        code = format!(
            "{}{}",
            code,
            KEYPAD[position.1 as usize][position.0 as usize]
        )
    }

    return code;
}