use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let files = [
        "./inputs/day4.test",
        "./inputs/day4.test2",
        "./inputs/day4.prod",
    ];
    for file in files {
        let grid = fs::read_to_string(file)?
            .lines()
            .map(|l| String::from(l))
            .collect_vec();
        let mut visited_x_cords: HashSet<(i32, i32)> = HashSet::new();
        let row: i32 = grid.len().try_into().unwrap();
        let col: i32 = grid.get(0).unwrap().len().try_into().unwrap();
        let mut count = 0;
        for r in 0..row {
            let row_input = grid.get(r as usize).unwrap();
            for c in 0..col {
                let char = row_input.chars().nth(c as usize).unwrap();
                if char != 'X' || visited_x_cords.contains(&(r, c)) {
                    continue;
                }
                let mut stack: VecDeque<char> = VecDeque::new();
                count += collect_dfs(&grid, &mut stack, r, c, row, col);
                visited_x_cords.insert((r, c));
            }
        }
        println!("{file}: {count}");
    }
    Ok(())
}
const DIRS: &'static [(i32, i32)] = &[
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, -1),
    (-1, 1),
];

const MAS: &'static [char] = &['M', 'A', 'S'];
fn collect_dfs(
    grid: &Vec<String>,
    stack: &mut VecDeque<char>,
    r: i32,
    c: i32,
    max_r: i32,
    max_c: i32,
) -> i32 {
    let mut met = 0;
    for (x, y) in DIRS {
        met += dfs(grid, stack, r, c, max_r, max_c, (*x, *y));
    }
    met
}

fn dfs(
    grid: &[String],
    stack: &mut VecDeque<char>,
    r: i32,
    c: i32,
    max_r: i32,
    max_c: i32,
    (x, y): (i32, i32),
) -> i32 {
    let mut met = 0;
    if stack.len() == 3 {
        return 1;
    }
    let which_xmas_char = stack.len();
    let xmas_char = MAS[which_xmas_char];
    let r = r + x;
    let c = c + y;
    if r < 0 || c < 0 || r >= max_r || c >= max_c {
        return 0;
    }
    let char = grid
        .get(r as usize)
        .unwrap()
        .chars()
        .nth(c as usize)
        .unwrap();
    if char != xmas_char {
        return 0;
    }
    stack.push_back(char);
    met += dfs(grid, stack, r, c, max_r, max_c, (x, y));
    stack.pop_back();
    met
}
