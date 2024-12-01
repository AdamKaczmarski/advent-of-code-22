use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let mut left = Vec::new();
    let mut right = Vec::new();
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
            right.push(right_num);
        });
    left.sort();
    right.sort();
    let sum: usize = left.iter().zip(right).map(|(r, l)| r.abs_diff(l)).sum();
    println!("{}", sum);

    Ok(())
}
