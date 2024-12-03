use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::fs;

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        tag("mul("),
        separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        ),
        tag(")"),
    )(input)
}

fn parse_instr(s: &str) -> IResult<&str, Option<Instruction>> {
    alt((
        map(parse_mul, |(x, y)| Some(Instruction::Mul(x, y))),
        map(tag("do()"), |_| Some(Instruction::Do)),
        map(tag("don't()"), |_| Some(Instruction::Dont)),
        map(map(anychar, drop), |_| None),
    ))(s)
}

fn main() -> Result<()> {
    let files = [
        "./inputs/day3.test",
        "./inputs/day3.test2",
        "./inputs/day3.prod",
    ];
    for file in files {
        let mut enabled = true;
        let sum = fs::read_to_string(file)?
            .lines()
            .map(|line| {
                let (_, intrs) = many0(parse_instr)(line).unwrap();
                intrs
            })
            .map(|x| {
                x.iter()
                    .filter_map(|i| match i {
                        Some(Instruction::Mul(x, y)) => {
                            if enabled {
                                Some(x * y)
                            } else {
                                None
                            }
                        }
                        Some(Instruction::Do) => {
                            enabled = true;
                            None
                        }
                        Some(Instruction::Dont) => {
                            enabled = false;
                            None
                        }
                        _ => None,
                    })
                    .sum::<i32>()
            })
            .sum::<i32>();
        println!("{}: {}", file, sum);
    }
    Ok(())
}
