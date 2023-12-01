
fn solve(mut input: String, part2: bool) -> u32 {
    if part2 {
        input = input
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "4")
            .replace("five", "5e")
            .replace("six", "6")
            .replace("seven", "7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
    }

    let mut result = 0;

    for line in input.lines() {
        let mut numbers = "".to_string();

        for char in line.chars() {
                if char.is_ascii_digit() {
                    numbers = format!("{}{}", numbers.chars().nth(0).unwrap_or(char), char);
                }
        }

        if numbers.len() == 1 { numbers = format!("{}{}", numbers, numbers) }
        result += numbers.parse::<u32>().unwrap();
    }
    return result;
}

fn main() {
    let input = include_str!("../../inputs/01.in").trim();

    print!("󰎤 {} \t", solve(input.to_string(), false));
    print!("󰎧 {} ",   solve(input.to_string(), true));
}
