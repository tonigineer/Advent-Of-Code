use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    for year in 2015..=2024 {
        let code_dir = format!("aoc_{}/src", year);
        let input_dir = format!("aoc_{}/inputs", year);

        match fs::create_dir(&input_dir) {
            Ok(_) => println!("Directory '{}' created successfully.", input_dir),
            Err(e) => eprintln!("Failed to create directory '{}': {}", input_dir, e),
        }

        for day in 1..=25 {
            let input_file = format!("{:02}.in", day);
            let input_path = Path::new(&current_dir).join(&input_dir).join(&input_file);

            let code_file = format!("day_{:02}.rs", day);
            let code_path = Path::new(&current_dir).join(&code_dir).join(&code_file);

            if code_path.exists() && !input_path.exists() {
                // Use aoc-cli crate to download input
                // https://docs.rs/crate/aoc-cli/latest
                let cmd = format!("aoc -q -y {year} -d {day} -I -i {} download", input_path.display());
                let _status =  Command::new("bash").arg("-c").arg(cmd).status();
            }
        }
    }
}
