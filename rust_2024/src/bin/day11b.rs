use std::{collections::HashMap, fs, time::Instant};

use anyhow::Result;

fn solve(file_name: &str, blinks: i32) -> Result<usize, anyhow::Error> {
    let input = fs::read_to_string(file_name)?;
    let stones = input
        .trim()
        .split(" ")
        .map(|x| (x.parse::<u128>().unwrap(), 1))
        .collect::<HashMap<_, _>>();
    let res = blinking_sim(stones, blinks);
    Ok(res)
}

fn number_length(mut n: u128) -> usize {
    let mut len = 0;
    while n > 0 {
        n /= 10;
        len += 1;
    }
    len
}

fn blinking_sim(mut stones: HashMap<u128, usize>, blinks: i32) -> usize {
    for _ in 1..=blinks {
        stones = blink(&stones);
    }
    stones.values().sum()
}

fn blink(old_stones: &HashMap<u128, usize>) -> HashMap<u128, usize> {
    let mut new_stones: HashMap<u128, usize> = HashMap::with_capacity(old_stones.len());
    for (&stone, &count) in old_stones {
        match stone {
            0 => {
                *new_stones.entry(1).or_default() += count;
            }
            _ => {
                if number_length(stone) % 2 == 0 {
                    let tmp = stone.to_string();
                    let (s1, s2) = tmp.split_at(tmp.len() / 2);
                    let s1 = s1.trim_start_matches('0');
                    let s2 = s2.trim_start_matches('0');
                    let mut r1 = 0;
                    let mut r2 = 0;
                    if s1.len() > 0 {
                        r1 = s1.parse::<u128>().unwrap();
                    }
                    if s2.len() > 0 {
                        r2 = s2.parse::<u128>().unwrap();
                    }
                    *new_stones.entry(r1).or_default() += count;
                    *new_stones.entry(r2).or_default() += count;
                } else {
                    *new_stones.entry(stone * 2024).or_default() += count;
                }
            }
        }
    }
    new_stones
}

fn main() -> Result<()> {
    let files = [
        "./inputs/day11.test",
        "./inputs/day11.test2",
        "./inputs/day11.prod",
    ];
    for file in files {
        let now = Instant::now();
        let res = solve(file, 75)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn blink_once() {
        let file = "./inputs/day11.test";
        let result = solve(file, 1).unwrap();
        assert_eq!(result, 7)
    }
    #[test]
    fn blink_example_6_times() {
        let file = "./inputs/day11.test2";
        let result = solve(file, 6).unwrap();
        assert_eq!(result, 22)
    }
    #[test]
    fn blink_example_25_times() {
        let file = "./inputs/day11.test2";
        let result = solve(file, 25).unwrap();
        assert_eq!(result, 55312)
    }
}
