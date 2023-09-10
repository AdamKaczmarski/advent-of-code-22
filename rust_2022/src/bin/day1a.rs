use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let max: usize = fs::read_to_string("./src/bin/day1.txt")?
        .split("\n\n")
        .map(|x| {
            x.lines().flat_map(str::parse::<usize>).sum()
        })
        .max()
        .unwrap();

    println!("max: {}", max);

    return Ok(());
}
