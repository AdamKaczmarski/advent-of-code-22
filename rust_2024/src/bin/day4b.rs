use std::fs;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let files = [
        "./inputs/day4.test",
        "./inputs/day4.test2",
        "./inputs/day4.test3",
        "./inputs/day4.prod",
    ];
    for file in files {
        let grid = fs::read_to_string(file)?
            .lines()
            .map(|l| String::from(l))
            .collect_vec();
        let row: i32 = grid.len().try_into().unwrap();
        let col: i32 = grid.get(0).unwrap().len().try_into().unwrap();
        let mut count = 0;
        for r in 0..row {
            let row_input = grid.get(r as usize).unwrap();
            for c in 0..col {
                let char = row_input.chars().nth(c as usize).unwrap();
                if char != 'A' {
                    continue;
                }
                if dfs(&grid, r, c, row, col) {
                    count += 1;
                }
            }
        }
        println!("{file}: {count}");
    }
    Ok(())
}
const DIRS: &'static [((i32, i32), (i32, i32))] = &[((-1, -1), (1, 1)), ((-1, 1), (1, -1))];
const MAS: &'static str = "MAS";
const SAM: &'static str = "SAM";

fn dfs(grid: &[String], r: i32, c: i32, max_r: i32, max_c: i32) -> bool {
    let mut matches = 0;
    //get string from cur position to
    // (-1,-1), (r,c) (1,1)
    // (-1,1), (r,c) (1,-1)
    // if starts if S check if matches MAS or SAM
    for ((x1, y1), (x2, y2)) in DIRS {
        if is_oob((r + x1, c + y1), (max_r, max_c)) || is_oob((r + x2, c + y2), (max_r, max_c)) {
            return false;
        }
        let char_top = grid
            .get((r + x1) as usize)
            .unwrap()
            .chars()
            .nth((c + y1) as usize)
            .unwrap();
        let char_mid = grid
            .get(r as usize)
            .unwrap()
            .chars()
            .nth(c as usize)
            .unwrap();
        let char_bot = grid
            .get((r + x2) as usize)
            .unwrap()
            .chars()
            .nth((c + y2) as usize)
            .unwrap();
        let s = format!("{}{}{}", char_top, char_mid, char_bot);
        if s == MAS || s == SAM {
            matches += 1;
        }
    }
    matches == 2
}

fn is_oob((r, c): (i32, i32), (max_r, max_c): (i32, i32)) -> bool {
    r < 0 || c < 0 || r >= max_r || c >= max_c
}
