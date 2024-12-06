use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

fn parse_line_to_vec(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect()
}

fn safe_check(report: &Vec<i32>) -> bool {
    // not needed for this puzzle but makes it work for more inputs
    if report.len() < 2 {
        return true;
    }

    let increase = report[0] < report[1];

    for window in report.windows(2) {
        let (prev, next) = (window[0], window[1]);

        if prev == next {
            return false;
        }
        if (prev - next).abs() > 3 {
            return false;
        }
        if increase && prev > next {
            return false;
        }
        if !increase && prev < next {
            return false;
        }
    }
    true
}

pub fn solve_parallel(lines: &Vec<String>) -> i32 {
    let safe_reports = lines
        .par_iter()
        .map(|line| {
            let report = parse_line_to_vec(line);
            safe_check(&report)
        })
        .filter(|&is_safe| is_safe)
        .count();

    safe_reports as i32
}

pub fn solve_sequential(lines: &Vec<String>) -> i32 {
    let mut safe_reports = 0;
    for line in lines {
        let report = parse_line_to_vec(&line);

        if safe_check(&report) {
            safe_reports += 1;
        }
    }
    safe_reports
}
