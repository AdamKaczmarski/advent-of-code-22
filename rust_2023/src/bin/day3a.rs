use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::{char, fs};

fn extract_numbers_from_line(line: &str) -> Vec<&str> {
    // I don't have the license to use regex but I struggled before that, ok
    // regexlicensing.org
    let re = Regex::new(r"([0-9]+)").unwrap();
    let res = re
        .find_iter(line)
        .map(|number_str| number_str.as_str())
        .collect_vec();
    println!("{:?}",res);
    return res
}

fn is_number_valid(start_idx: usize, l_to_take: usize, line: &str, check_line: &str) -> bool {
    let mut start = start_idx;
    let mut end = start_idx + l_to_take;
    if start_idx != 0 {
        //check left side INLINE
        start = start_idx - 1;
        let character: char = line.chars().nth(start).unwrap();
        if !character.is_digit(10) && character.ne(&'.') {
            return true;
        }
    }
    if end != line.len() - 1 {
        // check right side INLINE
        let character = line.chars().nth(end);
        match character {
            Some(x) => {
                if !x.is_digit(10) && x.ne(&'.') {
                    return true;
                }
            }
            None => (),
        }
        end = l_to_take + 1;
    }
    let count = check_line
        .chars()
        .skip(start)
        .take(end + 1)
        .filter(|c: &char| {
            // println!("char: {c}");
            return !c.is_digit(10) && c.ne(&'.');
        })
        .count();
    // println!("count: {count}");

    if count > 0 {
        return true;
    }

    return false;
}

fn get_valid_parsed_number(
    start_idx: usize,
    l_to_take: usize,
    line: &str,
    line_before: &str,
) -> usize {
    if is_number_valid(start_idx, l_to_take, line, line_before) {
        // println!(
        //     "valid: {}",
        //     line[start_idx..start_idx + l_to_take].to_owned()
        // );
        return line[start_idx..start_idx + l_to_take]
            .parse::<usize>()
            .unwrap();
    }

    return 0;
}
fn get_valid_sum_before_after(
    start_idx: usize,
    end_idx: usize,
    line: &str,
    line_before: &str,
    line_after: &str,
) -> usize {
    let before_check = get_valid_parsed_number(start_idx, end_idx, line, line_before);
    let after_check = get_valid_parsed_number(start_idx, end_idx, line, line_after);
    // println!("line: {} before val:{}, after val:{}", line, before_check, after_check);
    // println!("valid: {}", std::cmp::max(before_check, after_check));
    return std::cmp::max(before_check, after_check);
}

fn main() -> Result<()> {
    let mut sum: usize = 0;

    let text = fs::read_to_string("./src/bin/day3.prod")?;
    let lines_tmp = &text.lines().collect_vec();
    text.lines().enumerate().for_each(|(idx, line)| {
        let values: Vec<&str> = extract_numbers_from_line(line);

        for value in values {
            let start_idx = line.find(value).unwrap();
            let length_to_take = value.len();

            let mut line_before;
            let mut line_after;

            if idx == 0 {
                line_before = idx;
                line_after = idx + 1;
            } else {
                line_before = idx - 1;
                line_after = idx + 1;
            }
            if idx == lines_tmp.len() - 1 {
                line_after = idx;
                line_before = idx - 1;
            }

            if line_after != idx && line_before != idx {
                sum += get_valid_sum_before_after(
                    start_idx,
                    length_to_take,
                    line,
                    lines_tmp[line_before],
                    lines_tmp[line_after],
                );
            } else if line_after == idx {
                sum += get_valid_parsed_number(
                    start_idx,
                    length_to_take,
                    line,
                    lines_tmp[line_before],
                );
            } else if line_before == idx {
                sum +=
                    get_valid_parsed_number(start_idx, length_to_take, line, lines_tmp[line_after]);
            }
        }
    });
    println!("{}", sum);

    Ok(())
}
