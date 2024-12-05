use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let files = ["./inputs/day5.test", "./inputs/day5.prod"];
    for file in files {
        let input = fs::read_to_string(file)?;
        let parts = input.split("\n\n").collect_vec();
        let mut number_afters: HashMap<i32, HashSet<i32>> = HashMap::new();
        parts.get(0).unwrap().lines().for_each(|line| {
            let (x, y) = line
                .split('|')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            number_afters.entry(x).or_insert(HashSet::new()).insert(y);
        });
        let incorrect_orders: Vec<Vec<i32>> = parts
            .get(1)
            .unwrap()
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .filter(|ordering| !filter_for_correct_orders(ordering, &number_afters))
            .collect_vec();
        let mut sum = 0;
        for mut inorder in incorrect_orders {
            inorder.sort_by(|a, b| number_afters.entry(*b).or_default().contains(a).cmp(&true));
            sum += inorder[inorder.len() / 2];
        }
        println!("{file}: {sum}");
    }
    Ok(())
}

fn filter_for_correct_orders(
    ordering: &Vec<i32>,
    number_afters: &HashMap<i32, HashSet<i32>>,
) -> bool {
    //brute force solution
    for i in 0..ordering.len() - 1 {
        let x = ordering.get(i).unwrap();
        match number_afters.get(x) {
            Some(set) => {
                for j in i + 1..ordering.len() {
                    let y = ordering.get(j).unwrap();
                    if !set.contains(y) {
                        return false;
                    }
                }
            }
            None => return false,
        }
    }
    true
}
