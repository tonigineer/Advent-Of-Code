fn solve_part1(input: &str) -> usize {
    return input.chars().count() - 2 * input.matches(')').count();
}

fn solve_part2(input: &str) -> usize {
    let mut level = 0;
    let mut result: usize = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => panic!("Char not defined.")
        }
        result = i;
        if level < 0 {
            break;
        }
    }
    return result + 1;
}

fn main() {
    let input = include_str!("../../inputs/01.in").trim();

    print!("󰎤 {} ", solve_part1(&input));
    print!("󰎧 {} ", solve_part2(&input));
}

