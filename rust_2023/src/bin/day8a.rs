use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashMap, fs};

fn get_graph(node_input: &str) -> HashMap<&str, (&str, &str)> {
    let mut graph = HashMap::<&str, (&str, &str)>::new();
    node_input.lines().for_each(|node_def| {
        let node = &node_def[0..3];
        let left = &node_def[7..10];
        let right = &node_def[12..15];
        graph.insert(node, (left, right));
    });

    return graph;
}

fn solve(file_path: &str) -> Result<usize, anyhow::Error> {
    let input = fs::read_to_string(file_path)?;
    let input = input.split("\n\n").collect_vec();
    let instructions = input.get(0).unwrap().trim();
    let graph = get_graph(input.get(1).unwrap());
    let mut current_node = "AAA";
    let mut steps = 0;
    loop {
        for instruction in instructions.chars() {
            current_node = match instruction {
                'R' => graph.get(current_node).unwrap().1,
                'L' => graph.get(current_node).unwrap().0,
                _ => panic!("Wrong character here {}", instruction),
            };
            steps += 1;
        }
        if current_node == "ZZZ" {
            break;
        }
    }

    return Ok(steps);
}

fn main() -> Result<()> {
    let t1 = solve("./inputs/day8.test")?;
    let t2 = solve("./inputs/day8.test2")?;
    let p1 = solve("./inputs/day8.prod")?;

    println!("Test1: Actual={}, Expected={}, pass={}", t1, 2, t1 == 2);
    println!("Test2: Actual={}, Expected={}, pass={}", t2, 6, t2 == 6);
    println!("Prod Result: {}", p1);
    return Ok(());
}
