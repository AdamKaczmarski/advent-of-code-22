use anyhow::Result;
use std::{fs, str::FromStr, time::Instant};

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Velocity {
    dx: isize,
    dy: isize,
}

#[derive(Debug)]
struct Robot {
    p: Pos,
    v: Velocity,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    //p=0,4 v=3,-3
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let (p, v) = s
            .split_once(' ')
            .expect(&format!("Couldn't split line {}", s));
        let (x, y) = p
            .split_once(',')
            .expect(&format!("Couldn't split line {}", p));
        let (dx, dy) = v
            .split_once(',')
            .expect(&format!("Couldn't split line {}", v));
        let (_, x) = x.split_at(2);
        let (_, dx) = dx.split_at(2);
        let x = x.parse().expect(&format!("Couldn't parse {}", x));
        let y = y.trim().parse().expect(&format!("Couldn't parse y: {}", y));
        let dx = dx.parse().expect(&format!("Couldn't parse {}", dx));
        let dy = dy.trim().parse().expect(&format!("Couldn't parse {}", dy));
        Ok(Robot {
            p: Pos { x, y },
            v: Velocity { dx, dy },
        })
    }
}

type Input<Robot> = Vec<Robot>;

fn solve(file_name: &str, rows: usize, cols: usize, seconds: usize) -> Result<usize> {
    let mut input: Input<Robot> = read_input(file_name)?;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let vertical_line = cols / 2;
    let horizontal_line = rows / 2;
    for r in input.iter_mut() {
        //doesnt work returns negative remainders
        // let dx = (r.v.dx % cols as isize) as usize;
        // let dy = (r.v.dy % rows as isize) as usize;
        //works
        let dx = r.v.dx.rem_euclid(cols as isize) as usize;
        let dy = r.v.dy.rem_euclid(rows as isize) as usize;
        r.p.x = (r.p.x + seconds * dx) % cols;
        r.p.y = (r.p.y + seconds * dy) % rows;
        if r.p.y < horizontal_line {
            if r.p.x < vertical_line {
                q1 += 1;
            } else if r.p.x > vertical_line {
                q2 += 1;
            }
        } else if r.p.y > horizontal_line {
            if r.p.x < vertical_line {
                q3 += 1;
            } else if r.p.x > vertical_line {
                q4 += 1;
            }
        }
    }

    Ok(q1 * q2 * q3 * q4)
}
fn _print_grid(grid: &Vec<Vec<isize>>) {
    println!();
    for r in grid {
        println!("{:?}", r);
    }
    println!();
}

fn read_input(file_name: &str) -> Result<Input<Robot>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| Robot::from_str(line).expect(&format!("Couldn't parse line {}", line)))
        .collect())
}

fn main() -> Result<()> {
    let files = [
        ("./inputs/day14.test", 7, 11),
        ("./inputs/day14.prod", 103, 101),
    ];
    for (file, rows, cols) in files {
        let now = Instant::now();
        let res = solve(file, rows, cols, 100)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let file = "./inputs/day14.test";
        let rows = 7;
        let cols = 11;
        let result = solve(file, rows, cols, 100).unwrap();
        let expected = 12;
        assert_eq!(result, expected)
    }
    #[test]
    fn test_teleporting_robot() {
        let file = "./inputs/day14.test_single_robot";
        let rows = 7;
        let cols = 11;
        let result = solve(file, rows, cols, 5).unwrap();
        let expected = 0;
        assert_eq!(result, expected)
    }
    #[test]
    fn modulo() {
        let a: isize = -7;
        let b = 3;
        println!("1: {a} % {b} == {}", a % b);
        println!(
            "2: (rem_euclid): {a}.rem_euclid({b}) == {}",
            a.rem_euclid(b)
        );
        let a: isize = 7;
        let b = 3;
        println!("3: {a} % {b} == {}", a % b);
        println!(
            "4: (rem_euclid): {a}.rem_euclid({b}) == {}",
            a.rem_euclid(b)
        );
        let a: isize = -7;
        let b = -3;
        println!("5: {a} % {b} == {}", a % b);
        println!(
            "6: (rem_euclid): {a}.rem_euclid({b}) == {}",
            a.rem_euclid(b)
        );
        let a: isize = -94;
        let b = 11;
        println!("7: {a} % {b} == {}", a % b);
        println!(
            "8: (rem_euclid): {a}.rem_euclid({b}) == {}",
            a.rem_euclid(b)
        );
    }
}
