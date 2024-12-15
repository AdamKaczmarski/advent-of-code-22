use anyhow::Result;
use itertools::Itertools;
use std::{fs, time::Instant};

#[derive(Copy, Debug, Clone)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn resolve_move(self) -> (i32, i32) {
        match self {
            Move::Left => (0, -1),
            Move::Down => (1, 0),
            Move::Right => (0, 1),
            Move::Up => (-1, 0),
        }
    }
}

fn which_move(c: char) -> Move {
    match c {
        'v' => Move::Down,
        '<' => Move::Left,
        '^' => Move::Up,
        '>' => Move::Right,
        _ => panic!("Wrong char used! {c}!"),
    }
}
fn solve(file_name: &str) -> Result<usize> {
    let (mut grid, moves): (Vec<Vec<char>>, Vec<Move>) = read_input(file_name)?;
    let mut cur_pos = find_robot(&grid);
    grid[cur_pos.1][cur_pos.0] = '.';
    for m in moves {
        cur_pos = move_robot(&mut grid, cur_pos, m);
    }
    Ok(sum_box_cords(&grid))
}

fn move_robot(grid: &mut Vec<Vec<char>>, (r, c): (usize, usize), m: Move) -> (usize, usize) {
    let (dr, dc) = m.resolve_move();
    let nr = (r as i32 + dr) as usize;
    let nc = (c as i32 + dc) as usize;
    let next = grid[nr][nc];
    //if next pos is wall return pos
    if next == '#' {
        return (r, c);
    }
    //if next pos is dot walk there
    if next == '.' {
        return (nr, nc);
    }
    //if next pos is box
    // call move_robot with box as it's pos
    // once it's done return the pos before
    if next == 'O' {
        let (nnr, nnc) = move_robot(grid, (nr, nc), m);
        let is_wall_next = nnr == nr && nnc == nc;
        if is_wall_next {
            // no swaps made
            return (r,c);
        } else {
            grid[nnr][nnc] = 'O';
            grid[nr][nc]='.';
        }
    }
    (nr, nc)
}

fn sum_box_cords(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    grid.iter().enumerate().for_each(|(r_idx, row)| {
        row.iter().enumerate().for_each(|(c_idx, c)| {
            if *c == 'O' {
                sum += 100 * r_idx + c_idx;
            }
        })
    });
    sum
}

fn find_robot(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '@' {
                return (r, c);
            }
        }
    }
    panic!("COULDNT FIND ROBOT");
}

fn read_input(file_name: &str) -> Result<(Vec<Vec<char>>, Vec<Move>)> {
    let input = fs::read_to_string(file_name)?;
    let (g, m) = input
        .split_once("\n\n")
        .expect("Wrong input, couldn't find \\n\\n");
    let grid: Vec<Vec<char>> = g
        .split('\n')
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut moves = Vec::with_capacity(m.len());
    m.split('\n').for_each(|c| {
        c.chars().for_each(|m| moves.push(which_move(m)));
    });
    Ok((grid, moves))
}

fn main() -> Result<()> {
    let files = ["./inputs/day15.test", "./inputs/day15.prod"];
    for file in files {
        let now = Instant::now();
        let res = solve(file)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}
fn _print_grid(grid: &Vec<Vec<char>>) {
    println!();
    for r in grid {
        for c in r {
            print!("{:?}", c);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_move() {
        let file = "./inputs/day15.test_testingmoves";
        let _result = solve(file).unwrap();
    }
    #[test]
    fn small_example_calc() {
        let file = "./inputs/day15.test_calc";
        let result = solve(file).unwrap();
        let expected = 104;
        assert_eq!(result, expected)
    }
    #[test]
    fn example() {
        let file = "./inputs/day15.test";
        let result = solve(file).unwrap();
        let expected = 10092;
        assert_eq!(result, expected)
    }
}
