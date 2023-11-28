use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

fn run_day(day_num: &str) {
    // Copy files for python script that creates badges based on
    // existing files.
    let _ = fs::create_dir(format!("../{}/", &day_num));
    let _ = fs::copy(
        format!("src/bin/{}.rs", &day_num),
        format!("../{}/{}.rs", &day_num, &day_num),
    );

    // Compile and run solution
    let timer = Instant::now();
    let cmd = Command::new(format!("./target/release/{}", &day_num))
        .output()
        .unwrap();

    let elapsed_time = timer.elapsed().as_micros() as f64 / 1000.0;

    let mut color_code = 32;
    if elapsed_time > 500.0 { color_code = 31 }
    else if elapsed_time > 10.0 { color_code = 33 }
    else if elapsed_time > 1.0 { color_code = 37 };

    println!(
        "Day {} | \x1b[1;{color_code}m󰅒 {:8.3}ms \x1b[0m | {}",
        day_num,
        elapsed_time,
        String::from_utf8(cmd.stdout).unwrap()
    );
}

fn main() {
    Command::new("cargo")
        .args(&["build", "--release", "--all-features"])
        .output()
        .unwrap();

    for it in 1..=25 {
        let day_num = format!("{:0>2}", it);
        let full_filename = format!("./src/bin/{}.rs", day_num);
        let path = Path::new(&full_filename);

        if path.exists() {
            run_day(&day_num);
        }
    }
}
