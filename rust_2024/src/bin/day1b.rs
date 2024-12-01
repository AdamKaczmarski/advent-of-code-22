use anyhow::Result;
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let mut left = Vec::new();
    let mut right = HashMap::new();
    fs::read_to_string("./inputs/day1.prod")?
        .lines()
        .for_each(|line| {
            let mut split = line.trim().split("   ");
            let left_num = split
                .next()
                .expect("Missing left number")
                .parse::<usize>()
                .expect("Couldn't parse the number");
            left.push(left_num);
            let right_num = split
                .next()
                .expect("Missing left number")
                .parse::<usize>()
                .expect("Couldn't parse the number");
            *right.entry(right_num).or_insert(0) += 1;
        });
    let sum: usize = left.iter().map(|&l| l * right.get(&l).unwrap_or(&0)).sum();
    println!("{}", sum);
    Ok(())
}
