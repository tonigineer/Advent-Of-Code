use itertools::Itertools;

fn solve(input: &str, part2: bool) -> usize {
    let container: Vec<u32> = input.lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    let mut num_solutions = 0;
    for n in 1..container.len() {
        let num = container
            .iter()
            .combinations(n)
            .filter(|c| c.into_iter()
                .copied()
                .sum::<u32>() == 150
            )
            .count();
        num_solutions += num;
        if part2 && num > 0 { return num }
    }
    return num_solutions;
}

fn main() {
    let input = include_str!("../../inputs/17.in").trim();

    print!("󰎤 {} \t\t", solve(input, false));
    print!("󰎧 {} ", solve(input, true));
}
