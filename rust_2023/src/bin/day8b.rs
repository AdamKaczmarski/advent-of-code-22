use anyhow::Result;
use gcd::Gcd;
use itertools::Itertools;
use std::{collections::HashMap, fs};

#[derive(Clone, Debug)]
struct Node {
    value: String,
    distance: usize,
    has_found_end: bool,
}

#[derive(Clone, Debug)]
struct NodeTuple<'a> {
    left: &'a str,
    right: &'a str,
}

fn get_graph(node_input: &str) -> (Vec<Node>, HashMap<String, NodeTuple>) {
    let mut graph = HashMap::<String, NodeTuple>::new();
    let mut a_nodes: Vec<Node> = Vec::new();
    node_input.lines().for_each(|node_def| {
        let node = String::from(&node_def[0..3]);
        if node.ends_with('A') {
            a_nodes.push(Node {
                value: node.clone(),
                has_found_end: false,
                distance: 0,
            });
        }
        let left = &node_def[7..10];
        let right = &node_def[12..15];
        let nt = NodeTuple { left, right };
        graph.insert(node, nt);
    });

    return (a_nodes, graph);
}

fn solve(file_path: &str) -> Result<usize, anyhow::Error> {
    let input = fs::read_to_string(file_path)?;
    let input = input.split("\n\n").collect_vec();
    let instructions = input.get(0).unwrap().trim();
    let (mut a_nodes, graph) = get_graph(input.get(1).unwrap());
    let mut finish_count = 0;
    let mut steps = 0;
    loop {
        for instruction in instructions.chars() {
            steps += 1;
            for current_node in a_nodes.iter_mut() {
                if !current_node.has_found_end {
                    current_node.value = match instruction {
                        'R' => graph.get(&current_node.value).unwrap().right.to_string(),
                        'L' => graph.get(&current_node.value).unwrap().left.to_string(),
                        _ => panic!("Wrong character here {}", instruction),
                    };
                    if current_node.value.ends_with('Z') {
                        current_node.distance = steps;
                        current_node.has_found_end = true;
                        finish_count += 1;
                    }
                }
            }
            if finish_count == a_nodes.len() {
                break;
            }
        }
        if finish_count == a_nodes.len() {
            break;
        }
    }
    let steps = get_lcm(&a_nodes);

    return Ok(steps);
}
fn lcm(a: usize, b: usize) -> usize {
    return a * b / a.gcd(b);
}

fn get_lcm(a_nodes: &Vec<Node>) -> usize {
    let mut val: usize = a_nodes.get(0).unwrap().distance;
    for node in a_nodes.iter() {
        val = lcm(val, node.distance);
    }
    return val;
}

fn main() -> Result<()> {
    let t1 = solve("./inputs/day8.test3")?;
    println!("Test3: Actual={}, Expected={}, pass={}", t1, 6, t1 == 6);
    let p1 = solve("./inputs/day8.prod")?;
    println!("Prod Result: {}", p1);
    return Ok(());
}
