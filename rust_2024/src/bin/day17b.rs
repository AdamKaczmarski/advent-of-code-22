use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::{fs, ops::BitXor, time::Instant};

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    out: Vec<usize>,
}
impl Computer {
    fn insert_out(&mut self, v: usize) {
        self.out.push(v);
    }
    fn _out_to_joined_number(&self) -> usize {
        self.out
            .clone()
            .into_iter()
            .join("")
            .parse()
            .expect("Couldnt parse out to number")
    }
    fn out_to_comma_string(&self) -> String {
        self.out.clone().into_iter().join(",")
    }
    fn compute(&mut self, programs: Vec<usize>) {
        let mut instr_ptr = 0;
        while instr_ptr < programs.len() {
            let instr = parse_opcode(programs[instr_ptr]);
            let operand = programs[instr_ptr + 1];
            instr_ptr += 2;
            match instr {
                Instruction::Adv => self.adv(operand),
                Instruction::Bxl => self.bxl(operand),
                Instruction::Bst => self.bst(operand),
                Instruction::Jnz => {
                    if let Some(jump) = self.jnz(operand) {
                        instr_ptr = jump;
                    }
                }
                Instruction::Bxc => self.bxc(),
                Instruction::Out => self.out(operand),
                Instruction::Bdv => self.bdv(operand),
                Instruction::Cdv => self.cdv(operand),
            };
            self._print_registers();
        }
    }
    fn _print_registers(&self) {
        println!(
            "Register_A: {}, Register_B: {}, Register_C: {}",
            self.register_a, self.register_b, self.register_c
        );
    }
    //0
    fn adv(&mut self, operand: usize) {
        let numerator = self.register_a;
        let denominator = 2_u32.pow(self.resolve_combo(operand) as u32) as usize;
        let div = numerator / denominator;
        self.register_a = div;
    }
    //1
    fn bxl(&mut self, operand: usize) {
        self.register_b = self.register_b.bitxor(operand);
    }
    //2
    fn bst(&mut self, operand: usize) {
        let operand = self.resolve_combo(operand);
        self.register_b = operand % 8;
    }
    //3
    fn jnz(&mut self, operand: usize) -> Option<usize> {
        if self.register_a == 0 {
            return None;
        }
        Some(operand)
    }
    //4
    fn bxc(&mut self) {
        self.register_b = self.register_b.bitxor(self.register_c);
    }
    //5
    fn out(&mut self, operand: usize) {
        let operand = self.resolve_combo(operand);
        self.insert_out(operand % 8);
    }
    //6
    fn bdv(&mut self, operand: usize) {
        let numerator = self.register_a;
        let denominator = 2_u32.pow(self.resolve_combo(operand) as u32) as usize;
        let div = numerator / denominator;
        self.register_b = div;
    }
    //7
    fn cdv(&mut self, operand: usize) {
        let numerator = self.register_a;
        let denominator = 2_u32.pow(self.resolve_combo(operand) as u32) as usize;
        let div = numerator / denominator;
        self.register_c = div;
    }
    fn resolve_combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("COMPUTER CRASHING INVALID COMBO"),
            _ => panic!("INVALID OPERAND {operand}"),
        }
    }
}

type Input<T> = T;

fn solve(file_name: &str) -> Result<usize> {
    //TODO: So part2 is, find value for register_a so that computer.out == program
    let (mut computer, program): Input<(Computer, Vec<usize>)> = read_input(file_name)?;
    //TODO: simulate A or figure out the value
    computer.compute(program);
    Ok(0)
}

fn read_input(file_name: &str) -> Result<Input<(Computer, Vec<usize>)>> {
    let input = fs::read_to_string(file_name)?;
    let (registers, program) = input.split_once("\n\n").expect("Wrong input!");
    let re_registers = Regex::new(r"[0-9]+").unwrap();
    let re_program = Regex::new(r"[0-9]").unwrap();
    let registers = registers
        .split("\n")
        .map(|register| {
            let m = re_registers
                .find(register)
                .expect("No number found in register")
                .as_str();
            m.parse::<usize>().expect("Couldn't parse number to usize")
        })
        .collect_vec();
    let program = re_program
        .find_iter(program)
        .map(|p| p.as_str().parse::<usize>().expect("Couldn't parse digit"))
        .collect_vec();
    Ok((
        Computer {
            register_a: registers[0],
            register_b: registers[1],
            register_c: registers[2],
            out: Vec::new(),
        },
        program,
    ))
}

fn parse_opcode(op: usize) -> Instruction {
    match op {
        0 => Instruction::Adv,
        1 => Instruction::Bxl,
        2 => Instruction::Bst,
        3 => Instruction::Jnz,
        4 => Instruction::Bxc,
        5 => Instruction::Out,
        6 => Instruction::Bdv,
        7 => Instruction::Cdv,
        _ => panic!("WRONG OPCODE TO PARSE"),
    }
}

fn main() -> Result<()> {
    let files = ["./inputs/day17.test2", "./inputs/day17.prod"];
    for file in files {
        let now = Instant::now();
        let res = solve(file)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
        println!();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let file = "./inputs/day17.test2";
        let result = solve(file).unwrap();
        let expected = 117440;
        assert_eq!(result, expected)
    }

    #[test]
    fn instruction_1() {
        let mut c = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            out: Vec::new(),
        };
        let p = vec![2, 6];
        c.compute(p);
        assert_eq!(c.register_b, 1)
    }
    #[test]
    fn instruction_2() {
        let mut c = Computer {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            out: Vec::new(),
        };
        let p = vec![5, 0, 5, 1, 5, 4];
        c.compute(p);

        assert_eq!(c._out_to_joined_number(), 12)
    }
    #[test]
    fn instruction_3() {
        let mut c = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            out: Vec::new(),
        };
        let p = vec![0, 1, 5, 4, 3, 0];
        c.compute(p);

        assert_eq!(c.register_a, 0);
        assert_eq!(c._out_to_joined_number(), 42567777310);
    }
    #[test]
    fn instruction_4() {
        let mut c = Computer {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            out: Vec::new(),
        };
        let p = vec![1, 7];
        c.compute(p);

        assert_eq!(c.register_b, 26);
    }
    #[test]
    fn instruction_5() {
        let mut c = Computer {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            out: Vec::new(),
        };
        let p = vec![4, 0];
        c.compute(p);

        assert_eq!(c.register_b, 44354);
    }
    #[test]
    fn copy() {
        let mut c = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            out: Vec::new(),
        };
        let p = vec![0, 3, 5, 4, 3, 0];
        c.compute(p);
        println!();
        let mut c = Computer {
            register_a: 117440,
            register_b: 0,
            register_c: 0,
            out: Vec::new(),
        };
        let p = vec![0, 3, 5, 4, 3, 0];
        c.compute(p.clone());
        assert_eq!(c.out, p);
    }
}
