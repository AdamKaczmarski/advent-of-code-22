use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let files = ["./inputs/day8.test", "./inputs/day8.prod"];
    for file in files {
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
        let input: Vec<String> = fs::read_to_string(file)?
            .lines()
            .map(|line| String::from(line.trim()))
            .collect_vec();
        let max_y = input.len() as i32;
        let max_x = input[0].len() as i32;
        //group all antenas(x,y) by frequncy
        //then for each combination of antesnas
        // measure the distance between and
        // add do antena + distance
        // if it's in the board => insert into hashset as they must be unique
        let grouped_antenas = group_antenas(&input);
        for cords in grouped_antenas.values() {
            for &cord in cords.iter() {
                for &cord2 in cords.iter() {
                    if cord == cord2 {
                        continue;
                    }
                    let (dx, dy) = distance(cord, cord2);
                    let (mut x, mut y) = cord;
                    x = x - dx;
                    y = y - dy;
                    if x >= 0 && y >= 0 && x < max_x && y < max_y {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
        println!("{}: {}", file, antinodes.len());
    }

    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Antenna {
    LowerCharacter(char),
    UpperCharacter(char),
    Number(i8),
}

fn distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    (x2 - x1, y2 - y1)
}

fn group_antenas(input: &Vec<String>) -> HashMap<Antenna, Vec<(i32, i32)>> {
    let mut res = HashMap::new();
    input.iter().enumerate().for_each(|(row, line)| {
        for (col, c) in line.chars().enumerate() {
            let antenna = char_to_antenna(c);
            match antenna {
                Some(a) => res
                    .entry(a)
                    .or_insert(Vec::new())
                    .push((row as i32, col as i32)),
                None => continue,
            };
        }
    });
    res
}

fn char_to_antenna(c: char) -> Option<Antenna> {
    if c.is_digit(10) {
        return Some(Antenna::Number(c.to_digit(10).unwrap() as i8));
    }
    if c.is_ascii_uppercase() {
        return Some(Antenna::UpperCharacter(c));
    }
    if c.is_ascii_lowercase() {
        return Some(Antenna::LowerCharacter(c));
    }

    return None;
}
