use anyhow::Result;
use std::{fs, time::Instant};

type Input<T> = Vec<Vec<T>>;

fn solve(file_name: &str) -> Result<usize> {
    let input: Input<char> = read_input(file_name)?;
    let max_r = input.len();
    let max_c = input[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; max_c]; max_r];
    let mut res = 0;
    for r in 0..max_r {
        for c in 0..max_c {
            if visited[r][c] {
                continue;
            }
            let (area, perimeter) = dfs(&input, &mut visited, (r, c), (max_r, max_c));
            res += area * perimeter;
        }
    }
    Ok(res)
}

const DIRS: &'static [(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn dfs(
    input: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    (r, c): (usize, usize),
    bounds: (usize, usize),
) -> (usize, usize) {
    if visited[r][c] {
        return (0, 0);
    }
    visited[r][c] = true;
    let mut perimeter = 0;
    let mut area = 1;
    for &dir in DIRS {
        //border
        if will_be_oob((r, c), dir, bounds) {
            perimeter += 1;
            continue;
        }
        //neighbor is the same char
        let cur_c = input[r][c];
        let nr = r.checked_add_signed(dir.0).unwrap();
        let nc = c.checked_add_signed(dir.1).unwrap();
        let nei = input[nr][nc];
        if cur_c == nei {
            let (a, p) = dfs(input, visited, (nr, nc), bounds);
            area += a;
            perimeter += p;
        } else {
            perimeter += 1;
        }
    }
    (area, perimeter)
}

// This will check if underflow happens or if the cord is higher than max
fn will_be_oob(
    (r, c): (usize, usize),
    (dr, dc): (isize, isize),
    (max_r, max_c): (usize, usize),
) -> bool {
    let r = r.checked_add_signed(dr);
    let c = c.checked_add_signed(dc);
    r.is_none() || c.is_none() || r.unwrap() >= max_r || c.unwrap() >= max_c
}

fn read_input(file_name: &str) -> Result<Input<char>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn main() -> Result<()> {
    let files = ["./inputs/day12.test3", "./inputs/day12.prod"];
    for file in files {
        let now = Instant::now();
        let res = solve(file)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_example() {
        let file = "./inputs/day12.test";
        let result = solve(file).unwrap();
        let expected = 140;
        assert_eq!(result, expected)
    }
    #[test]
    fn region_inside_example() {
        let file = "./inputs/day12.test2";
        let result = solve(file).unwrap();
        let expected = 772;
        assert_eq!(result, expected)
    }
    #[test]
    fn example() {
        let file = "./inputs/day12.test3";
        let result = solve(file).unwrap();
        let expected = 1930;
        assert_eq!(result, expected)
    }
}
