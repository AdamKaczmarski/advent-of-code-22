use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
    let mut ways = 1;
    fs::read_to_string(&"./inputs/day6.prod")?
        .lines()
        .map(|x| {
            return x
                .split(" ")
                .filter(|s| return !(s.is_empty() || s.eq(&" ")))
                .skip(1)
                .map(|n| {
                    return n.parse::<usize>().unwrap();
                })
                .collect_vec();
        })
        .tuples()
        .for_each(|(times, distances)| {
            times.iter().zip(distances.iter()).for_each(|(x, y)| {
                let mut ways_counter = 0;
                for speed in 0..*x {
                    let remaining_race_time = x - speed;
                    match remaining_race_time * speed > *y {
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
        });
    println!("{:?}", ways);
    Ok(())
}
