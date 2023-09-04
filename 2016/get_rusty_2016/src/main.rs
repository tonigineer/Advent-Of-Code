use std::process::Command;
use std::path::Path;
use std::time::Instant;


fn run_day(day_num: &str) {
    let timer = Instant::now();

    let cmd = Command::new("cargo")
        .args(&["run", "--release", "--bin", &day_num])
        .output()
        .unwrap();
    let output = String::from_utf8(cmd.stdout).unwrap();

    println!("Day {} | {}󰅒 {}ms", day_num, output, timer.elapsed().as_millis());
}


fn main() {
    for it in 1..=25 {
        let day_num = format!("{:0>2}", it);
        let full_filename = format!("./inputs/{}.in", day_num);
        let path = Path::new(&full_filename);

        if path.exists() {
            run_day(&day_num);
        }
    }
}
