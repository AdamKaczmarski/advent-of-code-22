use anyhow::Result;
use std::{fs, time::Instant};

type Input<T> = Vec<Vec<T>>;

fn solve(file_name: &str) -> Result<usize> {
    let input: Input<char> = read_input(file_name)?;
    println!("{:?}", input);
    Ok(0)
}

fn read_input(file_name: &str) -> Result<Input<char>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn main() -> Result<()> {
    let files = ["./inputs/day12.test", "./inputs/day12.prod"];
    for file in files {
        let now = Instant::now();
        let res = solve(file)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let file = "./inputs/day12.test";
        let result = solve(file).unwrap();
        let expected = 140;
        assert_eq!(result, expected)
    }
}
