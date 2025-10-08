use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

pub fn read_puzzle_input(day: &u32, year: &u32) -> Result<String> {
    let file_path = generate_file_path(day, year);

    if !file_path.exists() {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if let Err(e) = download_input(day, year, &file_path) {
            eprintln!("download failed for {:02}-{} ({})", day, year, e);
        }
    }

    fs::read_to_string(&file_path)
}

fn generate_file_path(day: &u32, year: &u32) -> PathBuf {
    PathBuf::from(format!("inputs/year_{}/day_{:02}.in", year, day))
}

fn download_input(day: &u32, year: &u32, file_path: &Path) -> Result<()> {
    const SESSION_PATH: &str = "./adventofcode.session";

    if !Path::new(SESSION_PATH).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Please provide the Cookies in ./adventofcode.session",
        ));
    }

    let command = format!(
        "curl \"https://adventofcode.com/{}/day/{}/input\" \
         -s --fail --cookie \"session=$(<adventofcode.session)\"",
        year, day
    );

    let output = Command::new("bash").arg("-c").arg(command).output()?;
    if !output.status.success() {
        let status = output.status;
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::other(if stderr.is_empty() {
            format!("curl failed ({status})")
        } else {
            format!("curl failed ({status}) — {stderr}")
        }));
    }

    fs::write(file_path, &output.stdout)?;

    Ok(())
}
