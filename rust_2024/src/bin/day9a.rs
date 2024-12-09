use std::{fs, time::Instant};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
struct MemoryBlock {
    id: usize,
    value: usize,
}

#[derive(Debug, PartialEq)]
enum Segment {
    Block(MemoryBlock),
    Skip,
}

fn main() -> Result<()> {
    let files = ["./inputs/day9.test" , "./inputs/day9.prod"];
    for file in files {
        let now = Instant::now();
        let input = fs::read_to_string(file)?.trim().to_string();
        let mut file_system = parse_input_into_fs(input);
        let mut result = 0;
        let mut last_idx = file_system.len() - 1;
        while file_system[last_idx] == Segment::Skip {
            last_idx -= 1;
        }
        for i in 0..file_system.len() {
            if i > last_idx {
                break;
            }
            let s = &file_system[i];
            match s {
                Segment::Block(memory_block) => {
                    result += i * memory_block.id;
                }
                Segment::Skip => {
                    //move from last indx to skip
                    match &file_system[last_idx] {
                        Segment::Block(memory_block) => {
                            result += i * memory_block.id;
                            file_system[i] = Segment::Block(*memory_block);
                            file_system[last_idx] = Segment::Skip;
                            while file_system[last_idx] == Segment::Skip {
                                last_idx -= 1;
                            }
                        }
                        Segment::Skip => (),
                    }
                }
            }
        }
        println!("{}: {} in {}ms", file, result, now.elapsed().as_millis());
    }
    Ok(())
}

#[allow(dead_code)]
fn print_fs(fs: &Vec<Segment>) {
    for s in fs {
        match s {
            Segment::Block(memory_block) => print!("{}", memory_block.id),
            Segment::Skip => print!("."),
        }
    }
    println!();
}

fn parse_input_into_fs(input: String) -> Vec<Segment> {
    // println!("{input}");
    let mut aoc_fs = Vec::with_capacity(input.len());
    let mut block_id = 0;
    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();
        let c = c.to_digit(10).expect(&format!("Couldn't parse {}", c));
        if i % 2 == 0 {
            // println!("char: {c} => memory");
            //memory
            for _ in 0..c {
                aoc_fs.push(Segment::Block(MemoryBlock {
                    id: block_id,
                    value: c as usize,
                }));
            }
            block_id += 1;
        } else {
            //skips
            // println!("char: {c} => skip");
            for _ in 0..c {
                aoc_fs.push(Segment::Skip);
            }
        }
    }
    aoc_fs
}
