fn solve(input: &str, part2: bool) -> usize {
    let presents: usize = input.parse().unwrap();

    // Memorize intensive solution
    let mut houses: Vec<usize> = vec![0; presents/10];

    for elf in 1..houses.len(){
        for id in (elf..houses.len()).step_by(elf) {
            if part2 {
                if id / elf == 51 { break }
                houses[id] += elf * 11;
            } else { 
                houses[id] += elf * 10;
            }
        }
    }

    return houses.iter().position(|h| h >= &presents).unwrap();
}

fn main() {
    let input = include_str!("../../inputs/20.in").trim();

    print!("󰎤 {} ", solve(input, false));
    print!("󰎧 {} ", solve(input, true));
}
