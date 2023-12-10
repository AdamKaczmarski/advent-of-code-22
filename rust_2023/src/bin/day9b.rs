use anyhow::{Error, Result};
use itertools::Itertools;
use std::fs;

fn is_vec_only_zeros(x: &Vec<isize>) -> bool {
    for y in x.iter() {
        if *y != 0 as isize {
            return false;
        }
    }
    return true;
}

fn first_value_in_history(history: Vec<isize>) -> isize {
    if is_vec_only_zeros(&history) {
        return *history.last().unwrap();
    }
    let history2 = history
        .iter()
        .tuple_windows()
        .map(|(x, y)| x - y)
        .collect_vec();
    return history.first().unwrap() + first_value_in_history(history2);
}

fn solve(input_file: &str) -> Result<isize, Error> {
    let sum: isize = fs::read_to_string(input_file)?
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|y| {
                    return y.trim().parse::<isize>().unwrap();
                })
                .collect_vec()
        })
        .map(|x| first_value_in_history(x))
        .sum();
    return Ok(sum);
}

fn main() -> Result<()> {
    let t1 = solve("./inputs/day9.test")?;
    println!("Test1: Actual={}, Expected={}, pass={}", t1, 2, t1 == 2);
    let p1 = solve("./inputs/day9.prod")?;
    println!("Prod result: {}", p1);
    Ok(())
}
