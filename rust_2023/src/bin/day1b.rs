use anyhow::Result;
use std::fs;

const DIGITS_SPELLED: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn match_spld_digits(spelled: &str) -> char {
    return match spelled {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("Couldn't parse {}", spelled),
    };
}

fn find_spelled_idx(line: &str, reverse: bool, idx: &mut usize) -> Option<usize> {
    let mut found_len: usize = 0;
    for spld_dig in DIGITS_SPELLED {
        if reverse {
            match line.rfind(spld_dig) {
                Some(idx_f) => {
                    if idx_f >= *idx {
                        found_len = spld_dig.len();
                        *idx = idx_f;
                    }
                }
                None => continue,
            };
        } else {
            match line.find(spld_dig) {
                Some(idx_f) => {
                    if idx_f <= *idx {
                        found_len = spld_dig.len();
                        *idx = idx_f;
                    }
                }
                None => continue,
            };
        }
    }
    if found_len == 0 {
        return None;
    }
    return Some(found_len);
}

fn get_digit(line: &str, reverse: bool) -> char {
    let mut idx: usize;
    if reverse {
        idx = line.rfind(|c: char| c.is_digit(10)).unwrap_or_default();
    } else {
        idx = line.find(|c: char| c.is_digit(10)).unwrap_or_default();
    }
    return match find_spelled_idx(line, reverse, &mut idx) {
        None => line.chars().nth(idx).unwrap(),
        Some(len_found) => {
            let spld: &str;
            if reverse {
                spld = &line[idx..idx + len_found];
            } else {
                spld = &line[idx..idx + len_found];
            }
            match_spld_digits(spld)
        }
    };
}

fn main() -> Result<()> {
    let sum: i32 = fs::read_to_string("./src/bin/day1.prod")?
        .lines()
        .map(|line| {
            let mut digits = String::from(get_digit(line, false));
            digits.push(get_digit(line, true));

            return digits.parse::<i32>().unwrap();
        })
        .sum();
    println!("{}", sum);

    return Ok(());
}
