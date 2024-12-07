use std::fs;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

fn possibility(
    result: usize,
    numbers: &Vec<usize>,
    operation: Operator,
    mut cur: usize,
    idx: usize,
) -> bool {
    match numbers.get(idx) {
        Some(num) => {
            match operation {
                Operator::Add => cur += num,
                Operator::Multiply => cur *= num,
            }
            return possibility(result, numbers, Operator::Add, cur, idx + 1)
                || possibility(result, numbers, Operator::Multiply, cur, idx + 1);
        }
        None => {
            //Used all numbers -- basecase
            if cur == result {
                return true;
            }
            false
        }
    }
}

fn main() -> Result<()> {
    let files = ["./inputs/day7.test" , "./inputs/day7.prod"];
    for file in files {
        let result: usize = fs::read_to_string(file)?
            .lines()
            .map(|line| parse_equation(line))
            .filter(|(result, numbers)| {
                possibility(*result, numbers, Operator::Add, 0, 0)
                    || possibility(*result, numbers, Operator::Multiply, 0, 0)
            })
            .map(|(result, _)| result)
            .sum();
        println!("{file}: {}", result);
    }
    Ok(())
}

fn parse_equation(input: &str) -> (usize, Vec<usize>) {
    let (result, numbers) = input
        .split_once(":")
        .expect(&format!("Line not parsable: {}", input));
    let result = result
        .trim()
        .parse::<usize>()
        .expect(&format!("Not parsable int: {}", result));
    let numbers = numbers
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse::<usize>()
                .expect(&format!("Not parsable int: {}", result))
        })
        .collect_vec();
    (result, numbers)
}
