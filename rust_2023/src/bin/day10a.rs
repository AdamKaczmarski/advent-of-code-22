use anyhow::{Error, Result};
use itertools::Itertools;
use std::fs;

enum DirectionShort {
    North,
    East,
    South,
    West,
}

impl DirectionShort {
    fn get_direction_values(&self) -> Direction {
        return match self {
            DirectionShort::North => Direction { x: -1, y: 0 },
            DirectionShort::East => Direction { x: 0, y: 1 },
            DirectionShort::South => Direction { x: 1, y: 0 },
            DirectionShort::West => Direction { x: 0, y: -1 },
            // DirectionShort::North => Direction { x: 0, y: -1 },
            // DirectionShort::East => Direction { x: 1, y: 0 },
            // DirectionShort::South => Direction { x: 0, y: 1 },
            // DirectionShort::West => Direction { x: -1, y: 0 },
        };
    }
}

#[derive(Debug, Clone)]
struct Direction {
    x: isize,
    y: isize,
}

struct Position {
    x: isize,
    y: isize,
}
impl Direction {
    fn is_out(&self, pos: &Position, max_row: usize, max_col: usize) -> bool {
        //upper bounds
        if self.x + pos.x > max_row.try_into().unwrap()
            || self.y + pos.y > max_col.try_into().unwrap()
        {
            return true;
        }
        //lower bounds
        if self.x == -1 && pos.x == 0 || self.y == -1 && pos.y == 0 {
            return true;
        }
        return false;
    }
}

fn match_direction(tile: &char) -> Vec<Direction> {
    let mut moves = Vec::new();
    match tile {
        '|' => {
            moves.push(DirectionShort::North.get_direction_values());
            moves.push(DirectionShort::South.get_direction_values());
        }
        '-' => {
            moves.push(DirectionShort::East.get_direction_values());
            moves.push(DirectionShort::West.get_direction_values());
        }
        'L' => {
            moves.push(DirectionShort::North.get_direction_values());
            moves.push(DirectionShort::East.get_direction_values());
        }
        'J' => {
            moves.push(DirectionShort::North.get_direction_values());
            moves.push(DirectionShort::West.get_direction_values());
        }
        '7' => {
            moves.push(DirectionShort::South.get_direction_values());
            moves.push(DirectionShort::West.get_direction_values());
        }
        'F' => {
            moves.push(DirectionShort::South.get_direction_values());
            moves.push(DirectionShort::East.get_direction_values());
        }
        '.' => (),
        'S' => {
            moves.push(DirectionShort::North.get_direction_values());
            moves.push(DirectionShort::East.get_direction_values());
            moves.push(DirectionShort::South.get_direction_values());
            moves.push(DirectionShort::West.get_direction_values());
        }
        _ => panic!("Wrong character in input :("),
    };
    return moves;
}

fn filter_options(
    maze: &Vec<Vec<char>>,
    pos: (usize, usize),
    opts: &Vec<Direction>,
    seen: &Vec<Vec<bool>>,
) -> Vec<Direction> {
    let max_row = maze.len();
    let max_col = maze.get(0).unwrap().len();
    let pos_neg = Position {
        x: pos.0.try_into().unwrap(),
        y: pos.1.try_into().unwrap(),
    };
    let mut new_opts = Vec::new();
    for direction in opts.iter() {
        //index out of bound
        if direction.is_out(&pos_neg, max_row, max_col) {
            // println!("OUT pos:{:?}, dir:{:?}", pos,direction);
            continue;
        }
        //has seen
        if *has_seen(seen, (add(pos.0, direction.x), add(pos.1, direction.y))) {
            // println!("SEEN pos:{:?}, dir:{:?}", pos,direction);
            continue;
        }
        //ground
        let row: &Vec<char> = maze
            .get::<usize>((pos_neg.x + direction.x).try_into().unwrap())
            .unwrap();
        let new_tile: &char = row
            .get::<usize>((pos_neg.y + direction.y).try_into().unwrap())
            .unwrap();
        if new_tile == &'.' {
            // println!("GROUND pos:{:?}, dir:{:?}", pos,direction);
            continue;
        }
        new_opts.push(direction.clone());
    }
    return new_opts;
}

fn move_opts(maze: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<Direction> {
    let tile = get_tile(maze, pos);
    let moves_opts = match_direction(tile);
    return moves_opts;
}
fn add(u: usize, i: isize) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}
fn walk(
    maze: &Vec<Vec<char>>,
    pos: (usize, usize),
    seen: &mut Vec<Vec<bool>>,
    mut max: usize,
    counter: usize,
) -> usize {
    // println!("walk pos: {:?}", pos);
    //check if already seen point
    if counter > 1 && get_tile(maze,pos)==&'S'{
        return counter;
    }
    // if *has_seen(seen, pos) {
    //     return 1;
    // }
    let directions = move_opts(maze, pos);
    // println!("{:?} before filter {:?}", pos, directions);
    let directions = filter_options(maze, pos, &directions, seen);
    // println!(
    //     "for pos:{:?} those directions are available:{:?}",
    //     pos, directions
    // );
    if counter > max {
        max = counter.to_owned();
    }
    //mark position as seen
    *seen.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap() = true;
    // println!("{:?}", seen);

    for direction in directions {
        let new_pos = (add(pos.0, direction.x), add(pos.1, direction.y));
        // println!(
        //     "Walking from {:?} to {:?}",
        //     get_tile(maze, pos),
        //     get_tile(maze, new_pos)
        // );
        let candidate_max = walk(maze, new_pos, seen, max, counter+1);
        if candidate_max > max {
            max = candidate_max;
        }
    }

    return max;
}

fn has_seen(seen: &Vec<Vec<bool>>, pos: (usize, usize)) -> &bool {
    return seen.get(pos.0).unwrap().get(pos.1).unwrap();
}

fn get_tile(maze: &Vec<Vec<char>>, pos: (usize, usize)) -> &char {
    return maze.get(pos.0).unwrap().get(pos.1).unwrap();
}

fn solve(input_file: &str) -> Result<usize, Error> {
    let binding = fs::read_to_string(input_file)?;
    let input = binding.lines().collect_vec();
    let start = find_start_pos(&input);
    let input: Vec<Vec<char>> = binding
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    // println!("{:?}", input);

    let (rows, cols): (usize, usize) = get_row_col_length(&input);
    let mut seen = vec![vec![false; cols]; rows];
    let max = walk(&input, start, &mut seen, 0, 1);

    // println!("max: {:?}", max/2);

    return Ok(max/2);
}

fn main() -> Result<()> {
    let t1 = solve("./inputs/day10.test")?;
    println!("Test: Actual={:?}, Expected={:?}, pass={}", t1, 4, t1 == 4);
    let t2 = solve("./inputs/day10.test2")?;
    println!("Test2: Actual={:?}, Expected={:?}, pass={}", t2, 8, t2 == 8);
    //
    let p1 = solve("./inputs/day10.prod")?;
    println!("Prod result: {}", p1);

    Ok(())
}

fn find_start_pos(input: &Vec<&str>) -> (usize, usize) {
    let mut start = (0, 0);
    for (row, line) in input.iter().enumerate() {
        match line.find('S') {
            Some(col) => {
                start = (row, col);
                break;
            }
            None => continue,
        }
    }
    return start;
}

fn get_row_col_length(maze: &Vec<Vec<char>>) -> (usize, usize) {
    let rows = maze.len();
    let cols = maze.get(0).unwrap().len();
    return (rows, cols);
}
