use itertools::Itertools;

fn lookandsay(input: &str, repetitions: u8) -> usize {
    let mut process_string: String = String::from(input);

    for _ in 0..repetitions {
        let mut next_string: String = String::new();

        for (digit, group) in &process_string.chars().group_by(|c| *c) {
            let num: Vec<_> = group.collect();
            next_string.push_str(&num.len().to_string());
            next_string.push_str(&digit.to_string());
        }
        process_string = String::from(next_string);
    }
    return process_string.to_string().len();
}

fn main() {
    let input = include_str!("../../inputs/10.in").trim();

    print!("󰎤 {} ", lookandsay(&input, 40));
    print!("󰎧 {} ", lookandsay(&input, 50));
}
