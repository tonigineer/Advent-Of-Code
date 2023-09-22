fn part1(mut input: Vec<Vec<i32>>) -> i32 {
    let mut valid_triangle = 0;

    for item in input.iter_mut() {
        item.sort();
        if item[0] + item[1] > item[2] {
            valid_triangle += 1;
        }
    }

    return valid_triangle;
}


fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut valid_triangle = 0;

    for i in (0..input.len()).step_by(3) {
        for k in 0..=2 {
            valid_triangle += (
                input[i+0][k] + input[i+1][k] > input[i+2][k] &&
                input[i+0][k] + input[i+2][k] > input[i+1][k] &&
                input[i+2][k] + input[i+1][k] > input[i+0][k]
            ) as i32;
        }
    }

    return valid_triangle;
}


fn main() {
    let input = aoc::parse_multiple_items_per_line::<i32>("./inputs/03.in");

    print!("󰎤 {} ", part1(input.clone()));
    print!("󰎧 {} ", part2(input));
}
