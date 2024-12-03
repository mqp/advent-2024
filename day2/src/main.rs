use std::error::Error;
use std::io;

fn parse_report(line: &str) -> Result<Vec<isize>, Box<dyn Error>> {
    let mut levels = Vec::<isize>::new();
    for part in line.split_whitespace() {
        levels.push(part.parse()?)
    }
    Ok(levels)
}

fn is_safe_with_removal(report: &[isize]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let fixed_report = &report[0..i].chain(&report[i+1..report.len()]);
        if is_safe(&fixed_report) {
            return true;
        }
    }
    return false;
}

fn is_safe(report: &[isize]) -> bool {
    if report.len() < 2 {
        return true;
    }
    let mut seen_decreasing = false;
    let mut seen_increasing = false;
    for i in 0..report.len() - 1 {
        let curr = report[i];
        let next = report[i+1];
        let delta = curr - next;
        if delta > 0 {
            seen_increasing = true;
            if seen_decreasing {
                return false;
            }
        } else if delta < 0 {
            seen_decreasing = true;
            if seen_increasing {
                return false;
            }
        } else {
            // must be either monotonically increasing or decreasing
            return false;
        }
        if delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }
    }
    return true;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut safe_count = 0;
    for line in io::stdin().lines() {
        let report = parse_report(&line?)?;
        if is_safe_with_removal(&report) {
            safe_count += 1;
        }
    }
    println!("{}", safe_count);
    Ok(())
}
