fn solve(input: &str, num_zeros: usize) -> u32 {
    let mut index = 0;

    loop {
        let digest = md5::compute(format!("{input}{index}"));

        if &format!("{:?}", digest)[0..num_zeros] == format!("{:0num_zeros$}", 0) {
            return index;
        }

        index += 1
    }
}

fn main() {
    let input = include_str!("../../inputs/04.in").trim();

    print!("󰎤 {} \t", solve(&input, 5));
    print!("󰎧 {} ", solve(&input, 6));
}
