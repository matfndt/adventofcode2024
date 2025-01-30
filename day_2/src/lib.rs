use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

fn parse_line_to_vec(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect()
}

fn constraints(prev: i32, next: i32, increase: bool) -> bool {
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
    true
}

fn safe_check(report: &Vec<i32>) -> bool {
    // not needed for this puzzle but makes it work for more inputs
    if report.len() < 2 {
        return true;
    }

    let increase = report[0] < report[1];

    for window in report.windows(2) {
        let (prev, next) = (window[0], window[1]);

        if !constraints(prev, next, increase) {
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

// Part two

fn establish_direction(report: &[i32]) -> Option<(bool, usize, bool, i32, i32)> {
    if report.len() < 2 {
        return Some((true, report.len() - 1, false, report[0], report[0]));
    }

    let mut used_removal = false;
    let second_last_included;
    let mut last_included = report[0];
    let increase;

    for i in 1..report.len() {
        let curr = report[i];
        if curr != last_included {
            let diff = curr - last_included;
            if diff.abs() <= 3 {
                if diff > 0 {
                    increase = true;
                } else {
                    increase = false;
                }
                second_last_included = last_included;
                last_included = curr;
                return Some((
                    increase,
                    i,
                    used_removal,
                    last_included,
                    second_last_included,
                ));
            } else {
                if !used_removal {
                    used_removal = true;
                    continue;
                } else {
                    return None;
                }
            }
        } else {
            if !used_removal {
                used_removal = true;
                continue;
            } else {
                return None;
            }
        }
    }

    if report.len() == 1 {
        return Some((true, 0, used_removal, report[0], report[0]));
    } else {
        None
    }
}

fn safe_check_with_dampener(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let (increase, idx, mut used_removal, mut last_included, mut second_last_included) =
        match establish_direction(report) {
            Some(s) => s,
            None => return false,
        };

    for i in (idx + 1)..report.len() {
        let curr = report[i];
        if constraints(last_included, curr, increase) {
            second_last_included = last_included;
            last_included = curr;
        } else {
            if !used_removal {
                // let option_a = true;
                let mut option_b = false;
                if constraints(second_last_included, curr, increase) {
                    option_b = true;
                }

                if option_b {
                    used_removal = true;
                }
            } else {
                return false;
            }
        }
    }
    true
}

pub fn solve_with_dampener(lines: &Vec<String>) -> i32 {
    let mut safe_reports = 0;
    for line in lines {
        let report = parse_line_to_vec(line);
        if safe_check_with_dampener(&report) {
            safe_reports += 1;
        }
    }
    safe_reports
}
