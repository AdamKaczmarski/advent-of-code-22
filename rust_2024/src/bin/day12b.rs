use anyhow::Result;
use std::{fs, time::Instant};

type Input<T> = Vec<Vec<T>>;

fn solve(file_name: &str) -> Result<usize> {
    let input: Input<char> = read_input(file_name)?;
    println!("{:?}", input);
    let max_r = input.len();
    let max_c = input[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; max_c]; max_r];
    let mut res = 0;
    println!("{:?}", visited);
    for r in 0..max_r {
        for c in 0..max_c {
            if visited[r][c] {
                continue;
            }
            let (area, sides) = dfs(&input, &mut visited, (r, c), (max_r, max_c));
            println!(
                "CHAR: {:?}-{}, area: {}, sides: {}",
                (r, c),
                input[r][c],
                area,
                sides
            );
            res += area * sides;
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
    //I saw a hint that number of corners == number of sides
    let mut corners = 0;
    let mut area = 1;
    for &dir in DIRS {
        //border
        // println!("cord: {:?}, d:{:?}, bounds:{:?}",(r,c),dir,bounds);
        let mut oob_count = 0;
        for &dir2 in DIRS {
            if oob_count >= 2 {
                corners += 1;
                break;
            }
            if will_be_oob((r, c), dir2, bounds) {
                oob_count += 1;
            }
        }
        if will_be_oob((r, c), dir, bounds) {
            continue;
        }
        //neighbor is the same char
        let cur_c = input[r][c];
        let nr = r.checked_add_signed(dir.0).unwrap();
        let nc = c.checked_add_signed(dir.1).unwrap();
        let nei = input[nr][nc];
        if cur_c == nei {
            let (a, c) = dfs(input, visited, (nr, nc), bounds);
            area += a;
            corners += c;
        }
    }
    (area, corners)
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

// fn is_oob((r, c): (usize, usize)) -> bool {
//     r >= max_r || c >= max_c
// }

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
        let expected = 80;
        assert_eq!(result, expected)
    }
    #[test]
    fn example_xo() {
        let file = "./inputs/day12.test2";
        let result = solve(file).unwrap();
        let expected = 436;
        assert_eq!(result, expected)
    }
    #[test]
    fn example() {
        let file = "./inputs/day12.test3";
        let result = solve(file).unwrap();
        let expected = 1206;
        assert_eq!(result, expected)
    }
}
