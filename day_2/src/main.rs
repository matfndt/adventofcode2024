use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let path = &args[1];
    let file = File::open(path)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    // process in parallel with rayon
    let safe_reports = day_2::solve_parallel(&lines);
    println!("Number of safe reports: {}", safe_reports);

    // non parallel version
    let safe_reports = day_2::solve_sequential(&lines);
    println!("Number of safe reports: {}", safe_reports);

    Ok(())
}
