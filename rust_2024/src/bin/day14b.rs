use anyhow::Result;
use std::{
    fs,
    str::FromStr,
    thread::sleep,
    time::{Duration, Instant},
};

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
    // let mut grid = vec![vec![0; cols]; rows];
    for s in 1..=seconds {
        println!("SECONDS {s}");
        let mut grid = vec![vec![0; cols]; rows];
        for r in input.iter_mut() {
            //works
            let dx = r.v.dx.rem_euclid(cols as isize) as usize;
            let dy = r.v.dy.rem_euclid(rows as isize) as usize;
            r.p.x = (r.p.x + seconds * dx) % cols;
            r.p.y = (r.p.y + seconds * dy) % rows;
            grid[r.p.y][r.p.x] += 1;
        }
        if is_grid_unique(&grid) {
            println!("SOL :{}", s);
            break;
        }
        // _print_grid(&grid);
        // sleep(Duration::from_secs(2));
    }
    Ok(0)
}

fn is_grid_unique(grid: &Vec<Vec<i32>>) -> bool {
    for ele in grid {
        for &c in ele {
            if c > 1 {
                return false;
            }
        }
    }
    return true;
}
fn _print_grid(grid: &Vec<Vec<isize>>) {
    println!();
    for r in grid {
        for c in r {
            print!("{}", c);
        }
        print!("\n");
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
        // ("./inputs/day14.test", 7, 11),
        ("./inputs/day14.prod", 103, 101),
    ];
    for (file, rows, cols) in files {
        let now = Instant::now();
        let res = solve(file, rows, cols, 10000)?;
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
