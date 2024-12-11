use std::time::Instant;

use anyhow::Result;

fn solve(file_name: &str) -> usize {
    0
}

fn main() -> Result<()> {
    let files = ["./inputs/day10.test", "./inputs/day10.prod"];
    for file in files {
        let now = Instant::now();
        let res = solve(file);
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}
