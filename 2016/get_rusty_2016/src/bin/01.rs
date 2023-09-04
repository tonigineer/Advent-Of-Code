use std::fs;
use std::collections::HashSet;


const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];


fn solve(input: &String, short_circuit: bool) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    let mut heading: u8 = 0;

    let mut trail: HashSet<(i32, i32)> = HashSet::new();

    for cmd in input.trim().split(", ") {

        let turn = cmd.chars().nth(0).unwrap();
        heading = match turn {
            'L' => (heading + 1) % 4,
            'R' => (heading + 3) % 4,
            _ => panic!("Incorrect turn.")
        };

        let steps: usize = (&cmd[1..]).parse().unwrap();
        for _ in 0..steps {
            position.0 += DIRECTIONS[heading as usize].0;
            position.1 += DIRECTIONS[heading as usize].1;

            if !short_circuit {
                continue;
            }

            if trail.contains(&position) {
                return position.0.abs() + position.1.abs();
            }
            trail.insert(position);
        }
    }

    return position.0.abs() + position.1.abs();
}


fn main() {
    let input: String = fs::read_to_string("./inputs/01.in").
        expect("Could not read input file.");

    print!("󰎤 {} \t", solve(&input, false));
    print!("󰎧 {} \t", solve(&input, true));
}
