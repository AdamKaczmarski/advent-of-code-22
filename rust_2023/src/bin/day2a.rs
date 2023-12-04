use anyhow::Result;
use itertools::Itertools;

use std::{fs, str::FromStr};

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

#[derive(Debug, PartialEq)]
struct Reveal {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Reveal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut red: u8 = 0;
        let mut green: u8 = 0;
        let mut blue: u8 = 0;
        s.split(",").for_each(|cube_type| {
            cube_type
                .trim()
                .split(" ")
                .tuples()
                .for_each(|(val, color)| match color {
                    "red" => red = val.parse().unwrap(),
                    "blue" => blue = val.parse().unwrap(),
                    "green" => green = val.parse().unwrap(),
                    _ => panic!("Couldn't parse {:?}", color),
                });
        });


        return Ok(Reveal { red, blue, green });
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    max_red: u8,
    max_blue: u8,
    max_green: u8,
}

impl Game {
    fn is_game_possible(&self) -> bool {
        return self.max_red <= MAX_RED && self.max_green <= MAX_GREEN && self.max_blue <= MAX_BLUE;
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        //"Game <num>:"
        let colon_idx = s.find(":").unwrap();
        let id: usize = s[5..colon_idx].parse().unwrap();
        let mut max_red: u8 = 0;
        let mut max_blue: u8 = 0;
        let mut max_green: u8 = 0;
        //"Game <num>: "
        s[colon_idx + 1..].split(";").for_each(|rev| {
            let reveal: Reveal = rev.trim().parse().unwrap();
            compare_and_set_max(&reveal.red, &mut max_red);
            compare_and_set_max(&reveal.green, &mut max_green);
            compare_and_set_max(&reveal.blue, &mut max_blue);
        });

        return Ok(Game {
            id,
            max_red,
            max_blue,
            max_green,
        });
    }
}

fn compare_and_set_max(value: &u8, max: &mut u8) {
    if value > max {
        *max = *value;
    }
}

fn main() -> Result<()> {
    let sum: usize = fs::read_to_string("./inputs/day2.prod")?
        .lines()
        .map(|line| {
            let game: Game = line.parse().unwrap();
            if game.is_game_possible() {
                return game.id;
            }
            return 0;
        })
        .sum();

    println!("{sum}");
    Ok(())
}
