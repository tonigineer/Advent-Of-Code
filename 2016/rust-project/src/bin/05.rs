use md5;


fn part1(id: &String) -> String {
    let mut hash = String::new();

    for i in 0.. {
        let digest = md5::compute(format!("{id}{i}"));

        // Indexing returns one byte, therefore two chars, e.g., 0xFF
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            hash.push(
                format!("{:x}", digest[2] & 0x0f)
                    .chars().nth(0).unwrap()
            );
            if hash.len() == 8 {
                break;
            }
        }
    }

    return hash;
}


fn main() {
    let input = aoc::parse_one_item_per_line::<String>("./inputs/05.in");

    print!("󰎤 {} ", part1(input.first().unwrap()));
    print!("󰎧 {} ", "NA");
}
