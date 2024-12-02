use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
    let files = ["./inputs/day2.test" , "./inputs/day2.prod"];
    for file in files {
        let count = fs::read_to_string(file)?
            .lines()
            .map(|line| {
                line.trim()
                    .split(" ")
                    .map(|x| x.parse().expect("Couldn't parse"))
                    .collect()
            })
            .filter(|report| is_safe_with_perms(&report))
            .count();
        println!("{}: {}", file, count);
    }

    Ok(())
}

fn is_safe_with_perms(report: &Vec<usize>) -> bool {
    (0..report.len()).any(|idx| {
        let mut y = report.clone();
        y.remove(idx);
        is_safe(&y)
    })
}

fn is_safe(report: &Vec<usize>) -> bool {
    let increasing = report.windows(2).all(|w| w[0] <= w[1]);
    let decreasing = report.windows(2).all(|w| w[0] >= w[1]);
    if !increasing && !decreasing {
        return false;
    }
    report
        .iter()
        .tuple_windows()
        .all(|(x, y)| 1 <= y.abs_diff(*x) && y.abs_diff(*x) <= 3)
}
