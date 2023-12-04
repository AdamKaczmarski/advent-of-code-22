use anyhow::Error;
use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn solve(file: &str) -> Result<usize, Error> {
    let mut card_copies: [usize; 1000] = [1; 1000];
    let mut read_lines = 0;
    fs::read_to_string(file)?
        .lines()
        .enumerate()
        .for_each(|(idx, line): (usize, &str)| {
            read_lines += 1;
            let numbers: Vec<&str> = line[7..].split("|").collect_vec();
            let mut matches: usize = 0;
            for winner in numbers.get(0).unwrap().trim().split(" ") {
                for scratched in numbers.get(1).unwrap().trim().split(" ") {
                    if scratched.eq("") {
                        continue;
                    }
                    if winner == scratched {
                        matches += 1;
                    }
                }
            }
            let copies_of_current_card = card_copies[idx];
            for i in idx + 1..idx + 1 + matches {
                if copies_of_current_card == 0 {
                    card_copies[i] += 1
                } else {
                    card_copies[i] += 1 * copies_of_current_card;
                }
            }
        });
    let mut sum=0;
    for i in 0..read_lines{
        sum+=card_copies[i];
    }

    return Ok(sum);
}

fn main() -> Result<()> {
    println!("Test: {}", solve("./inputs/day4.test").unwrap());
    println!("Prod: {}", solve("./inputs/day4.prod").unwrap());
    Ok(())
}
