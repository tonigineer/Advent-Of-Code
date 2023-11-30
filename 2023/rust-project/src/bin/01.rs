
fn solve(input: &str, part2: bool) -> i32 {
    return 0;
}

fn main() {
    let input = include_str!("../../inputs/01.in").trim();

    print!("󰎤 {} \t", solve(&input, false));
    print!("󰎧 {} ",   solve(&input, true));
}
