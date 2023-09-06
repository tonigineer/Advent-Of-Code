use std::fs;


fn part1(input: &String) -> i32 {
    let mut valid_triangle = 0;

    for line in input.lines() {
        let mut sides: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        sides.sort();

        if sides[0] + sides[1] > sides[2] {
            valid_triangle += 1;
        }
    }

    return valid_triangle;
}


fn part2(input: &String) -> i32 {
    let mut valid_triangle = 0;
    let mut list: Vec<Vec<i32>> = vec!();

    for line in input.lines() {
        list.push(
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        );
    }

    for i in (0..list.len()).step_by(3) {
        for k in 0..=2 {
            valid_triangle += (
                list[i+0][k] + list[i+1][k] > list[i+2][k] &&
                list[i+0][k] + list[i+2][k] > list[i+1][k] &&
                list[i+2][k] + list[i+1][k] > list[i+0][k]) as i32;
        }
    }

    return valid_triangle;
}


fn main() {
    let input: String = fs::read_to_string("./inputs/03.in").
        expect("Could not read input file.");

    // let list = aoc::parse_one_item_per_line::<i32>("./inputs/test.in");

    // for it in list.iter() {
    //     println!("{}", it);
    // }

    // let list2 = aoc::parse_multiple_items_per_line::<i32>("./inputs/06.in");
    // for it in list2.iter() {
    //     println!("{}", it[0]);
    // }

    print!("󰎤 {} ", part1(&input));
    print!("󰎧 {} ", part2(&input));
}
