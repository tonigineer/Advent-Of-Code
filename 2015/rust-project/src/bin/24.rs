use itertools::Itertools;

fn solve(input: &str, compartments: usize) -> usize {
    let packages: Vec<usize> = input.lines().map(|p| p.parse().unwrap()).collect();

    let total_weight: usize = packages.iter().sum::<usize>() / compartments;
    let max_packages_front = packages.len()/compartments;

    let mut entanglement = 0;
    for i in 1..=max_packages_front {
        let c = packages.iter().copied()
            .combinations(i)
            .filter(|c| c.iter().sum::<usize>() == total_weight)
            .sorted_by_key(|c| c.iter().fold(1, |a, b| a * *b as i32))
            .min();

        if ! c.is_none() {
            entanglement = c.unwrap().iter().fold(1, |a, b| a * b);
            break
        }
    }
    return entanglement;
}

fn main() {
    let input = include_str!("../../inputs/24.in").trim();

    print!("󰎤 {} \t", solve(input, 3));
    print!("󰎧 {} ", solve(input, 4));
}
