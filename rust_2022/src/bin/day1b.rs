use std::fs;
use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let mut maxes: Vec<usize> = fs::read_to_string("./src/bin/day1.txt")?
        .split("\n\n")
        .map(|x| x.lines().flat_map(str::parse::<usize>).sum())
        .collect_vec();

    maxes.sort_by(|a, b| b.cmp(a));

    let sum: usize = maxes.iter().take(3).sum();

    println!("Top 3 calories summed: {}", sum);

    return Ok(());
}
