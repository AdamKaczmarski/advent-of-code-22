use std::collections::BTreeSet;
use std::{fs, time::Instant};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
struct StoredFile {
    length: usize,
    fs_idx: usize,
}

impl StoredFile {
    fn checksum(&self, id: usize) -> usize {
        (self.fs_idx..self.fs_idx + self.length)
            .map(|i| i * id)
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SkipLength {
    length: usize,
    fs_idx: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Block {
    File(StoredFile),
    Skip(SkipLength),
}


// thanks u/jinschoi for his submission on Reddit. Helped me a lot to udnerstand
fn main() -> Result<()> {
    let files = ["./inputs/day9.test", "./inputs/day9.prod"];
    for file in files {
        let now = Instant::now();
        let input = fs::read_to_string(file)?.trim().to_string();
        let file_system = parse_input_into_fs(input);
        // print_fs(&file_system);
        let mut files: Vec<StoredFile> = Vec::new();
        //spaces indexed by size and orderd by index of appearance
        let mut spaces: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); 10];
        for f in file_system {
            match f {
                Block::File(stored_file) => {
                    files.push(stored_file);
                }
                Block::Skip(skip_length) => {
                    if skip_length.length > 0 {
                        spaces[skip_length.length].insert(skip_length.fs_idx);
                    }
                }
            }
        }
        for id in (0..files.len()).rev() {
            // find file with id
            let file = &mut files[id];
            // first first empty slot larger than length and below idx
            let empty = (file.length..10)
                .flat_map(|length| {
                    let free_idxs = &spaces[length];
                    free_idxs
                        .range(0..file.fs_idx)
                        .map(move |&free_idx| (length, free_idx))
                })
                .min_by_key(|(_, idx)| *idx);
            if let Some((length, empty_idx)) = empty {
                // remove empty idx and add a new, smaller one if necessary.
                spaces[length].remove(&empty_idx);
                if length > file.length {
                    spaces[length - file.length].insert(empty_idx + file.length);
                }
                // adjust file's idx
                file.fs_idx = empty_idx;
                continue;
            }
        }
        let result: usize = files.iter().enumerate().map(|(i, f)| f.checksum(i)).sum();

        println!("{}: {} in {}ms", file, result, now.elapsed().as_millis());
    }
    Ok(())
}

fn parse_input_into_fs(input: String) -> Vec<Block> {
    let mut aoc_fs: Vec<Block> = Vec::new();
    let mut idx = 0;
    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();
        let c = c.to_digit(10).expect(&format!("Couldn't parse {}", c));
        if i % 2 == 0 {
            //memory
            aoc_fs.push(Block::File(StoredFile {
                fs_idx: idx,
                length: c as usize,
            }));
        } else {
            //skips
            aoc_fs.push(Block::Skip(SkipLength {
                fs_idx: idx,
                length: c as usize,
            }));
        }
        idx += c as usize;
    }
    aoc_fs
}
