#![feature(linked_list_cursors)]
use std::{collections::LinkedList, fs, time::Instant};

use anyhow::Result;

fn solve(file_name: &str, blinks: i32) -> Result<usize, anyhow::Error> {
    let input = fs::read_to_string(file_name)?;
    let mut stones: LinkedList<String> = input.trim().split(" ").map(|x| x.to_string()).collect();
    // println!("{:?}", stones);
    blinking_sim(&mut stones, blinks);
    Ok(stones.len())
}

fn blinking_sim(stones: &mut LinkedList<String>, blinks: i32) {
    // println!("start {:?}", stones);
    for i in 1..=blinks {
        let mut cursor = stones.cursor_front_mut();
        while let Some(curr) = cursor.current() {
            if curr == &"0" {
                cursor.insert_before(String::from("1"));
                cursor.remove_current();
            } else if curr.len() % 2 == 0 {
                let (s1, s2) = curr.split_at(curr.len() / 2);
                let mut s1 = s1.trim_start_matches('0').to_string();
                let mut s2 = s2.trim_start_matches('0').to_string();
                if s1.len() == 0 {
                    s1 = "0".to_owned();
                }
                if s2.len() == 0 {
                    s2 = "0".to_owned();
                }
                cursor.insert_before(s1);
                cursor.insert_before(s2);
                cursor.remove_current();
            } else {
                let mut number = curr
                    .parse::<usize>()
                    .expect(&format!("Couldn't parse {}", curr));
                number *= 2024;
                let number = number.to_string();
                cursor.insert_before(number);
                cursor.remove_current();
            }
        }
        // println!("After {} blink", i);
        // println!("{:?}", stones);
    }
}

fn main() -> Result<()> {
    let files = [
        "./inputs/day11.test",
        "./inputs/day11.test2",
        "./inputs/day11.prod",
    ];
    for file in files {
        let now = Instant::now();
        let res = solve(file, 25)?;
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
