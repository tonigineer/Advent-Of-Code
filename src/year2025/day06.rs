//! # Trash Compactor
//!
//! TODO: Refactor and optimize

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.trim().lines().rev();
    let operations: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let mut result = vec![0u64; operations.len()];

    while let Some(l) = lines.next() {
        let numbers: Vec<u64> = l.split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();
        for idx in 0..operations.len() {
            result[idx] = match operations[idx] {
                "*" => result[idx].max(1) * numbers[idx],
                "+" => result[idx] + numbers[idx],
                _ => unreachable!(),
            };
        }
    }

    result.iter().sum::<u64>()
}

pub fn part2(input: &str) -> i64 {
    let mut lines = input.lines().rev();
    let operations: Vec<&str> = lines.next().unwrap().split_whitespace().rev().collect();

    let mut result = 0;

    let mut chars = Vec::new();
    for line in lines.rev() {
        let v: Vec<char> = line.chars().collect();
        chars.push(v);
    }

    // dbg!(&chars);
    dbg!(&operations);
    let mut op_idx = 0;
    let mut numbers = Vec::new();
    for idx in (0..chars[0].len()).rev() {
        let mut num = String::new();
        for r in &chars {
            if r[idx] != ' ' {
                num.push(r[idx]);
            }
        }
        // dbg!(&num);
        if !num.is_empty() {
            numbers.push(num.parse::<i64>().unwrap());
            if idx > 0 {
                continue;
            }
        }
        dbg!(&numbers);
        dbg!(&operations[op_idx]);
        match operations[op_idx] {
            "+" => {
                println!("PLUS");
                result += numbers.iter().sum::<i64>()
            }
            "*" => {
                println!("MUILT");
                result += numbers.iter().product::<i64>()
            }
            _ => unreachable!(),
        }
        dbg!(&result);
        op_idx += 1;
        numbers.clear()
    }
    result
}
