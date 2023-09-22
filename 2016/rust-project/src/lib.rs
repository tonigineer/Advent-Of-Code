// use anyhow::Result;
use std::str::FromStr;


pub fn parse_one_item_per_line<T>(path: &str) -> Vec<T>
where
    T: FromStr,
{
    std::fs::read_to_string(path)
        .expect(&format!("Could not read file {}", path))
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}


pub fn parse_multiple_items_per_line<T>(path: &str) -> Vec<Vec<T>>
where
    T: FromStr,
{
    std::fs::read_to_string(path)
        .expect(&format!("Could not read file {}", path))
        .lines()
        .map(|line| line
            .split_whitespace()
            .filter_map(|num| num.parse::<T>().ok()
            ).collect()
        ).collect()
}
