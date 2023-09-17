use nom::{
    bytes::complete::tag,
    character::complete::{u32, alpha0},
    multi::separated_list0,
    IResult,
};

use std::collections::HashMap;


fn parse(input: &str) -> IResult<&str, (Vec<&str>, u32, &str)> {
    let (input, names) = separated_list0(tag("-"), alpha0)(input)?;
    let (input, id) = u32(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, checksum) = alpha0(input)?;

    return Ok((input, (names[0..names.len()-1].to_vec(), id, checksum)));
}


fn decrypt(names: Vec<&str>, id: u32) -> bool {
    let shift: u8 = (id % 26) as u8;

    let mut decrypted_name = String::new();
    for name in names {
        for c in name.as_bytes() {
            let ord = c - b'a';
            decrypted_name.push(((ord + shift) % 26 + b'a') as char)
        }
        decrypted_name.push(' ');
    }

    if decrypted_name.contains("northpole") {
        return true;
    }

    return false;
}



fn solve(input: Vec<String>, part2: bool) -> u32 {
    let mut sum_of_ids: u32 = 0;

    for line in input {
        let result = parse(&line);
        let (names, id, checksum_must) = result.unwrap().1;

        if part2 {
            if decrypt(names, id) {
                return id;
            }
            continue;
        }

        // Count appearance of chars into dict
        let mut letter: HashMap<char, i32> = HashMap::new();
        for name in names {
            // println!("{}", name);
            for c in name.chars() {
                *letter.entry(c).or_insert(0) += 1;
            }
        }

        // Convert dict to vector with tuple and sort by count and than by char
        let mut v: Vec<(&char, &i32)> = letter.iter().collect();
        v.sort_by(|a, b| {
            if b.1 == a.1 {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(a.1)
            }

        });

        let mut checksum: String = String::new();
        for (k, _) in v.iter().take(checksum_must.len()) {
            checksum = format!("{checksum}{k}");
        }

        // println!("{letter:?} {checksum:?} {checksum_must}");

        if checksum == checksum_must {
            sum_of_ids += id;
        }
    }

    return sum_of_ids;
}




fn main() {
    let input = aoc::parse_one_item_per_line::<String>("./inputs/04.in");

    print!("󰎤 {} ", solve(input.clone(), false));
    print!("󰎧 {} ", solve(input, true));
}
