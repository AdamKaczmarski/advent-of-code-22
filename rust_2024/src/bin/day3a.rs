use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Mul {
    x: isize,
    y: isize,
}
impl Mul {
    fn resolve(&self) -> isize {
        self.x * self.y
    }
}
fn main() -> Result<()> {
    let files = ["./inputs/day3.test", "./inputs/day3.prod"];
    for file in files {
        let result: isize = fs::read_to_string(file)?
            .lines()
            .map(|line| {
                parse_muls_regex(line)
                    .iter()
                    .map(|mul| mul.resolve())
                    .sum::<isize>()
            })
            .sum();
        println!("{}: {}", file, result);
    }

    Ok(())
}

//day3b was solved with nom
fn parse_muls_regex(s: &str) -> Vec<Mul> {
    let remuls = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    remuls
        .captures_iter(s)
        .map(|capture| {
            let x = capture["x"].parse::<isize>().unwrap();
            let y = capture["y"].parse::<isize>().unwrap();
            Mul { x, y }
        })
        .collect_vec()
}
