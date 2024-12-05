use std::{fs, io};

/// Loads the input file for a specific day and year of the Advent of Code challenges.
///
/// # Arguments
///
/// - `day` (`u32`): The day of the puzzle (typically 1 to 25).
/// - `year` (`u32`): The year of the Advent of Code event.
///
/// # Returns
///
/// Returns an `io::Result<String>` which is:
/// - `Ok(String)` containing the contents of the input file if successful.
/// - `Err(io::Error)` if the file could not be read.
///
/// # Example
///
/// ```rust
/// use common::input::load_input;
///
/// fn main() -> std::io::Result<()> {
///     let input = load_input(1, 2023).unwrap_or_else(|_| "File not under source control ;)".to_string());
///     println!("{}", input);
///     Ok(())
/// }
/// ```
///
/// # File Path Format
///
/// The function constructs the input file path using the format:
///
/// ```text
/// aoc_{year}/inputs/{day}.in
/// ```
///
/// - `{year}`: The provided year.
/// - `{day}`: The day padded with zeros to ensure two digits (e.g., `01`, `02`, ..., `25`).
///
/// **Example:** For `day = 1` and `year = 2023`, the file path will be:
///
/// ```text
/// aoc_2023/inputs/01.in
/// ```
pub fn load_input(day: u32, year: u32) -> io::Result<String> {
    let input_path = format!("aoc_{}/inputs/{:0>2}.in", year, day);
    fs::read_to_string(input_path)
}
