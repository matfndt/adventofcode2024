use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use day_1::{calc_sum_of_differences, calc_weighted_sum};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() == 2 {
                    if let (Ok(left_num), Ok(right_num)) =
                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        left_list.push(left_num);
                        right_list.push(right_num);
                    } else {
                        eprintln!("Failed to parse line: {}", line);
                    }
                } else {
                    eprintln!("Line does not contain two numbers: {}", line);
                }
            } else {
                eprintln!("Failed to open file");
            }
        }
    }

    println!("Loaded both list as integer arrays");
    println!("Left list length: {:?}", left_list.len());
    println!("Right list length: {:?}", right_list.len());

    let result = calc_sum_of_differences(&mut left_list, &mut right_list);
    println!("Result Part 1: {}", result);

    let result = calc_weighted_sum(&left_list, &right_list);
    println!("Result Part 2: {}", result);

    Ok(())
}
