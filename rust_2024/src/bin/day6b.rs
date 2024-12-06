use std::{collections::HashSet, fs};

use anyhow::Result;

#[derive(Copy, Debug, Clone, PartialEq, Hash, Eq)]
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

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
struct CordDir {
    x: i32,
    y: i32,
    dir: Direction,
}

fn main() -> Result<()> {
    let files = ["./inputs/day6.test", "./inputs/day6.prod"];
    for file in files {
        let input = fs::read_to_string(file)?;
        let input: Vec<String> = input
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|str| String::from(str))
            .collect();
        let Some((x, y)) = find_start(&input) else {
            panic!("Start not found");
        };
        let start_char = input[y as usize].chars().nth(x as usize).unwrap();
        let dir = which_dir(start_char);
        //brute forcing this first, go over each tile and put the obstacle, check if cycle then
        let mut path = Vec::new();
        walk(&input, (x, y), dir, &mut path);
        let start = CordDir { x, y, dir };
        //when I was testing it I met some cords that were duplicated because of different
        //directions
        let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
        for cord_dir in path {
            if cord_dir.x == start.x && cord_dir.y == start.y {
                continue;
            }
            if has_cycle(&input, &start, (cord_dir.x, cord_dir.y)) {
                obstacles.insert((cord_dir.x, cord_dir.y));
            }
        }
        //1. get walking path till oob
        //2. for each tile do has cycle
        //detecting cycle: if the same obstacle was hit twice with the from the same direction

        println!("{file}: {}", obstacles.len());
    }
    Ok(())
}

fn has_cycle(input: &Vec<String>, start: &CordDir, obstacle: (i32, i32)) -> bool {
    let mut cur = start.clone();
    let mut already_hit_obstacles: HashSet<CordDir> = HashSet::new();
    loop {
        if cur.x < 0 || cur.y < 0 || cur.y >= (input.len() as i32) || cur.x >= (input[0].len() as i32) {
            return false;
        }
        let mut c = input[cur.y as usize].chars().nth(cur.x as usize).unwrap();
        let (x1, y1) = cur.dir.resolve_dir();
        if cur.x == obstacle.0 && cur.y == obstacle.1 {
            c = '#';
        }
        match match_tile(c) {
            Tile::Wall => {
                if already_hit_obstacles.contains(&cur) {
                    return true;
                }
                already_hit_obstacles.insert(cur);
                cur.x -= x1;
                cur.y -= y1;
                cur.dir = cur.dir.rot_right();
            }
            Tile::Dot => {
                cur.x += x1;
                cur.y += y1;
            }
        }
    }
}

fn walk(
    input: &Vec<String>,
    (mut x, mut y): (i32, i32),
    mut dir: Direction,
    path: &mut Vec<CordDir>,
) {
    loop {
        if x < 0 || y < 0 || x >= (input.len() as i32) || y >= (input[0].len() as i32) {
            break;
        }
        let c = input[y as usize].chars().nth(x as usize).unwrap();
        let (x1, y1) = dir.resolve_dir();
        match match_tile(c) {
            Tile::Wall => {
                x -= x1;
                y -= y1;
                dir = dir.rot_right();
            }
            Tile::Dot => {
                path.push(CordDir { x, y, dir });
                x += x1;
                y += y1;
            }
        }
    }
}

fn match_tile(c: char) -> Tile {
    match c {
        '.' | 'v' | '<' | '^' | '>' => Tile::Dot,
        '#' | '+' => Tile::Wall,
        _ => panic!("Wrong tile {c}"),
    }
}

fn find_start(input: &Vec<String>) -> Option<(i32, i32)> {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let c = input[i].chars().nth(j).unwrap();
            match c {
                'v' | '<' | '^' | '>' => return Some((j as i32, i as i32)),
                _ => {
                    continue;
                }
            }
        }
    }
    None
}
