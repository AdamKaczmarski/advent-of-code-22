use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

use anyhow::Result;
use itertools::Itertools;
fn main() -> Result<()> {
    let files = ["./inputs/day10.test", "./inputs/day10.prod"];
    for file in files {
        let now = Instant::now();
        let map = fs::read_to_string(file)?
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect_vec()
            })
            .collect_vec();
        let mut trails: HashMap<(i32, i32), i32> = HashMap::new();
        let max_r = map.len();
        let max_c = map[0].len();
        for r in 0..max_r {
            for c in 0..max_c {
                let h = map[r][c];
                if h != 0 {
                    continue;
                }
                let r = r as i32;
                let c = c as i32;
                let start = (r, c);
                walk(
                    &map,
                    &mut trails,
                    (r + 1, c),
                    h,
                    (max_r as i32, max_c as i32),
                    start,
                );
                walk(
                    &map,
                    &mut trails,
                    (r - 1, c),
                    h,
                    (max_r as i32, max_c as i32),
                    start,
                );
                walk(
                    &map,
                    &mut trails,
                    (r, c + 1),
                    h,
                    (max_r as i32, max_c as i32),
                    start,
                );
                walk(
                    &map,
                    &mut trails,
                    (r, c - 1),
                    h,
                    (max_r as i32, max_c as i32),
                    start,
                );
            }
        }
        let result: i32 = trails.values().sum();
        println!("{}: {} in {}ms", file, result, now.elapsed().as_millis());
    }
    Ok(())
}

fn walk(
    map: &Vec<Vec<i32>>,
    trails: &mut HashMap<(i32, i32), i32>,
    (r, c): (i32, i32),
    prev_h: i32,
    (max_r, max_c): (i32, i32),
    start: (i32, i32),
) {
    if is_oob((r, c), (max_r, max_c)) {
        return;
    }
    let h = map[r as usize][c as usize];
    if h == 0 && r == start.0 && c == start.1 {
        return;
    }
    if h == 9 && h - prev_h == 1 {
        *trails.entry(start).or_insert(0) += 1;
        return;
    }
    if h - prev_h == 1 {
        walk(map, trails, (r + 1, c), h, (max_r, max_c), start);
        walk(map, trails, (r - 1, c), h, (max_r, max_c), start);
        walk(map, trails, (r, c + 1), h, (max_r, max_c), start);
        walk(map, trails, (r, c - 1), h, (max_r, max_c), start);
    }
}

fn is_oob((r, c): (i32, i32), (max_r, max_c): (i32, i32)) -> bool {
    r < 0 || c < 0 || r >= max_r || c >= max_c
}
