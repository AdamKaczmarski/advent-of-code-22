use anyhow::Result;
use std::fs;

fn get_digit(line: &str, reverse: bool) -> char {
    let idx: usize;
    if reverse {
        idx = line.rfind(|c: char| c.is_digit(10)).unwrap();
    } else {
        idx = line.find(|c: char| c.is_digit(10)).unwrap();
    }
    return line.chars().nth(idx).unwrap();
}

fn main() -> Result<()> {
    let sum: i32 = fs::read_to_string("./src/bin/day1.prod")?
        .lines()
        .map(|line| {
            let mut digits = String::from(get_digit(line, false));
            digits.push(get_digit(line, true));

            return digits.parse::<i32>().unwrap();
        })
        .sum();
    println!("{}", sum);

    return Ok(());
}
