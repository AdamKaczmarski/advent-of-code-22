use std::{collections::HashSet, fs};

use anyhow::Result;

#[derive(Copy, Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn resolve_dir(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
        }
    }

    fn rot_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn which_dir(c: char) -> Direction {
    match c {
        'v' => Direction::Down,
        '<' => Direction::Left,
        '^' => Direction::Up,
        '>' => Direction::Right,
        _ => panic!("Wrong char used! {c}!"),
    }
}

enum Tile {
    Wall,
    Dot,
}

fn main() -> Result<()> {
    let files = ["./inputs/day6.test", "./inputs/day6.prod"];
    for file in files {
        let input = fs::read_to_string(file)?;
        let input: Vec<&str> = input.split("\n").collect();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let Some((mut x, mut y)) = find_start(&input) else {
            panic!("Start not found");
        };
        let start_char = input[y as usize].chars().nth(x as usize).unwrap();
        let mut dir = which_dir(start_char);
        loop {
            if x < 0 || y < 0 || x >= (input.len() as i32) || y >= (input[0].len() as i32) {
                break;
            }
            let c = input[y as usize].chars().nth(x as usize).unwrap();
            let (x1,y1) = dir.resolve_dir();
            match match_tile(c) {
                Tile::Wall => {
                    x -= x1;
                    y -= y1;
                    dir = dir.rot_right();
                }
                Tile::Dot => {
                    if !visited.contains(&(x,y)) {
                        visited.insert((x,y));
                    }
                    x += x1;
                    y += y1;
                }
            }
        }
        println!("{file}: {}", visited.len());
    }
    Ok(())
}

fn match_tile(c: char) -> Tile {
    match c {
        '.' | 'v' | '<' | '^' | '>' => Tile::Dot,
        '#' => Tile::Wall,
        _ => panic!("Wrong tile {c}"),
    }
}

fn find_start(input: &Vec<&str>) -> Option<(i32, i32)> {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let c = input[i].chars().nth(j).unwrap();
            match c {
                'v' | '<' | '^' | '>' => return Some((j as i32,i as i32)),
                _ => {
                    continue;
                }
            }
        }
    }
    None
}
