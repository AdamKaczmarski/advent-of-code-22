use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
    let count = fs::read_to_string("./inputs/day2.prod")?
        .lines()
        .map(|line| {
            line.trim()
                .split(" ")
                .map(|x| x.parse().expect("Couldn't parse"))
                .collect()
        })
        .filter(|report| is_safe(&report))
        .count();
    println!("{}", count);

    Ok(())
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
