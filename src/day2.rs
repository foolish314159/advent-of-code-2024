use std::{cmp::Ordering, fs::read_to_string};

use itertools::Itertools;

pub fn safe_reports(filename: &str, allow_violation: bool) -> usize {
    let input = read_to_string(filename).unwrap_or(String::from(""));

    input
        .lines()
        .filter(|&report| is_safe(report, allow_violation))
        .count()
}

fn is_safe(report: &str, allow_violation: bool) -> bool {
    let levels = report
        .split_whitespace()
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    if is_safe_levels(&levels) {
        return true;
    }

    if !allow_violation {
        return false;
    }

    for i in 0..levels.len() {
        let mut new_levels = levels.clone();
        new_levels.remove(i);
        let new_report = new_levels.iter().join(" ");
        if is_safe(&new_report, false) {
            return true;
        }
    }

    false
}

fn is_safe_levels(levels: &Vec<i32>) -> bool {
    // Assume there is atleast two levels in each report, otherwise this will panic
    // Would a report with only one level be considered safe or unsafe?
    let order = levels[0].cmp(&levels[1]);
    // Unsafe if gap between elements not atleast 1 (equal)
    if order == Ordering::Equal {
        return false;
    }

    for i in 0..levels.len() - 1 {
        // Unsafe if order changes
        if levels[i].cmp(&levels[i + 1]) != order {
            return false;
        }

        // Unsafe if gap between elements too big
        if (levels[i] - levels[i + 1]).abs() > 3 {
            return false;
        }
    }

    return true;
}
