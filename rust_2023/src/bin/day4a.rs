use anyhow::Error;
use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn solve(file: &str) -> Result<usize, Error> {
    let sum: usize = fs::read_to_string(file)?
        .lines()
        .map(|line: &str| {
            let numbers: Vec<&str> = line[7..].split("|").collect_vec();
            let mut sum: usize = 0;
            for winner in numbers.get(0).unwrap().trim().split(" ") {
                for scratched in numbers.get(1).unwrap().trim().split(" ") {
                    if scratched.eq("") {
                        continue;
                    }
                    if winner == scratched {
                        if sum == 0 {
                            sum = 1;
                        } else {
                            sum *= 2;
                        }
                    }
                }
            }
            return sum;
        })
        .sum();

    return Ok(sum);
}

fn main() -> Result<()> {
    println!("Test: {}", solve("./inputs/day4.test").unwrap());
    println!("Prod: {}", solve("./inputs/day4.prod").unwrap());
    Ok(())
}
