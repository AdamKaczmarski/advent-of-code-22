use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
    let mut ways = 1;
    fs::read_to_string(&"./inputs/day6.prod")?
        .lines()
        .map(|x| {
            let number = x.find(|c: char| c.is_digit(10)).unwrap();
            let y = &x[number..]
                .split(" ")
                .filter(|s| !(s.is_empty() || s.eq(&"")))
                .join("")
                .parse::<usize>()
                .unwrap();
            return y.clone();
        })
        .tuples()
        .for_each(|(x, y)| {
            let mut ways_counter = 0;
            for speed in 0..x {
                let remaining_race_time = x - speed;
                match remaining_race_time * speed > y {
                    true => {
                        ways_counter += 1;
                    }
                    false => (),
                }
            }
            if ways_counter != 0 {
                ways *= ways_counter;
            }
        });
    println!("{:?}", ways);
    Ok(())
}
