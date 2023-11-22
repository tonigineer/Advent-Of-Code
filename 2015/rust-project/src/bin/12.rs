use serde_json::Value;

fn solve(json: Value, part2: bool) -> i64 {
    if json.is_number() {
        return json.as_i64().unwrap();
    }

    if json.is_string() {
        return 0;
    }

    if json.is_array() {
        let mut sum_array = 0;
        for v in json.as_array().unwrap() {
            sum_array += solve(v.clone(), part2);
        }
        return sum_array;
    }

    if json.is_object() {
        let mut sum_object = 0;
        for (_, v) in json.as_object().unwrap() {
            if part2 {
                if v == "red" {
                    return 0;
                }
            }
            sum_object += solve(v.clone(), part2);
        }
        return sum_object;
    }

    return 0;
}

fn main() {
    let input = include_str!("../../inputs/12.in").trim();
    let data: Value = serde_json::from_str(input).unwrap();

    print!("󰎤 {} ", solve(data.clone(), false));
    print!("󰎧 {} ", solve(data, true));
}
