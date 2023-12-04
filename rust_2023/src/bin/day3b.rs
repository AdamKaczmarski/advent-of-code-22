use anyhow::Result;
use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone, Default)]
struct Number {
    start_idx: usize,
    end_idx: usize,
    value_str: String,
    value: usize,
}

impl Number {
    fn parse_value(&mut self) {
        self.value = self.value_str.parse::<usize>().unwrap();
    }
}

#[derive(Debug, Default)]
struct Symbol {
    idx: usize,
}

#[derive(Debug, Default)]
struct Line {
    line_no: usize,
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}
impl Line {
    fn set_line(&mut self, n: usize) {
        self.line_no = n;
    }
}

fn main() -> Result<()> {
    let mut lines: Vec<Line> = vec![];
    fs::read_to_string("./inputs/day3.prod")?
        .lines()
        .enumerate()
        .for_each(|(idx, line)| {
            let mut whole_line: Line = Line::default();
            whole_line.set_line(idx);
            let chars = line.chars().collect_vec();

            let mut number: Number = Number::default();
            for (c_idx, char) in chars.iter().enumerate() {
                if char.is_digit(10) {
                    number.end_idx = c_idx;
                    number.value_str.push(char.to_owned());
                } else if !char.is_digit(10) && char.eq(&'*') {
                    whole_line.symbols.push(Symbol { idx: c_idx });
                }
                if c_idx > 0 {
                    match line.chars().nth(c_idx - 1) {
                        Some(val) => {
                            if val.is_digit(10)
                                && ((char.eq(&'.') || char.is_ascii_punctuation())
                                    || (val.is_digit(10) && c_idx == chars.len() - 1))
                            {
                                number.start_idx = number.end_idx - (number.value_str.len() - 1);
                                number.parse_value();
                                whole_line.numbers.push(number.clone());
                                number = Number::default();
                            }
                        }
                        None => (),
                    }
                }
            }
            lines.push(whole_line);
        });

    let mut sum: usize = 0;
    for line in &lines {
        for symbol in &line.symbols {
            let mut adjs: Vec<Number> = vec![];
            if line.line_no != 0 && line.line_no != lines.len() - 1 {
                let line_after: &Line = lines.get(line.line_no + 1).unwrap();
                get_adjacent_numbers(symbol, line_after, &mut adjs);
                let line_before: &Line = lines.get(line.line_no - 1).unwrap();
                get_adjacent_numbers(symbol, line_before, &mut adjs);
            }
            for number in &line.numbers {
                if number.start_idx != 0 {
                    if number.start_idx - 1 == symbol.idx {
                        adjs.push(number.clone());
                    }
                }
                if number.end_idx + 1 == symbol.idx {
                    adjs.push(number.clone());
                }
            }
            if adjs.len() == 2 {
                sum += adjs.get(0).unwrap().value * adjs.get(1).unwrap().value;
            }
        }
    }
    println!("sum: {}", sum);
    Ok(())
}

fn get_adjacent_numbers(symbol: &Symbol, check_line: &Line, adjs: &mut Vec<Number>) {
    for number in &check_line.numbers {
        if number.start_idx == 0 {
            if number.start_idx <= symbol.idx && symbol.idx <= number.end_idx + 1 {
                adjs.push(number.clone());
            }
        } else {
            if number.start_idx - 1 <= symbol.idx && symbol.idx <= number.end_idx + 1 {
                adjs.push(number.clone());
            }
        }
    }
}
