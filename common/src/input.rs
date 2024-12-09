use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::{fs, io, io::Write};

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

    if !Path::new(&input_path).exists() {
        match download_input(&day, &year, &input_path) {
            Ok(_) => println!(
                "\x1b[35m Downloading\x1b[0m Input for Day \x1b[30m{}\x1b[0m of Year \x1b[30m{}\x1b[0m was successfully downloaded.",
                day, year
            ),
            Err(e) => eprintln!("Error downloading file: {}", e),
        }
    }

    fs::read_to_string(&input_path)
}

/// Downloads the Advent of Code input file for a given day and year if it's not already present.
///
/// This function attempts to retrieve the puzzle input from the Advent of Code website using `curl`.
/// It requires a session cookie to be stored in `$HOME/adventofcode.session`.
///
/// # Arguments
///
/// - `day` (`&u32`): The day of the puzzle (1 to 25).
/// - `year` (`&u32`): The year of the Advent of Code event.
/// - `input_path` (`&String`): The file path where the input should be saved.
///
/// # Returns
///
/// Returns `Result<(), Box<dyn Error>>`:
/// - `Ok(())` if the file was successfully downloaded and written.
/// - `Err(...)` if any error occurs, such as missing session file, failed network call, or I/O error.
///
/// # Requirements
///
/// - Ensure `$HOME/adventofcode.session` contains a valid AoC session cookie.
/// - `curl` must be installed and available on the system's PATH.
///
/// # Example
///
/// ```rust,ignore
/// if let Err(e) = download_input(&1, &2024, &"aoc_2024/inputs/01.in".to_string()) {
///     eprintln!("Failed to download input: {}", e);
/// } else {
///     println!("Input downloaded successfully!");
/// }
/// ```
fn download_input(
    day: &u32,
    year: &u32,
    input_path: &String,
) -> Result<(), Box<dyn Error + 'static>> {
    let home_dir = std::env::var("HOME")?;
    let session_path = format!("{}/adventofcode.session", home_dir);

    if !Path::new(&session_path).exists() {
        return Err("Please provide the Cookies in $HOME/adventofcode.session".into());
    }

    let command = format!(
        "curl \"https://adventofcode.com/{}/day/{:0>1}/input\" \
    -s --fail --cookie \"session=$(<{}/adventofcode.session)\"",
        year, day, home_dir
    );

    let output = Command::new("bash").arg("-c").arg(command).output()?;

    let mut file = fs::File::create(input_path)?;
    file.write_all(&output.stdout)?;

    Ok(())
}
