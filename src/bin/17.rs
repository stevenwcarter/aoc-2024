advent_of_code::solution!(17);

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct Registers {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

#[derive(Debug, Clone, PartialEq)]
struct OpCodeProgram {
    registers: Registers,
    program: Vec<u8>,
    output: Vec<u64>,
    position: usize,
    halt: bool,
}

// hacked up my copy of the compute function to return true/false as candidate
// options for the outputs
fn compute(program: &[u8], inst_position: usize, reg_a: u64) -> bool {
    let mut registers: [u64; 3] = [reg_a, 0, 0];

    for position in 0..(program.len() / 2) {
        let left = program[position * 2];
        let right = program[position * 2 + 1] as u64;
        let operand = get_operand(&right, &registers);
        match left {
            0 => {
                // adv - division
                registers[0] >>= operand;
            }
            1 => {
                // bxl - bitwise XOR B and operand
                registers[1] ^= right & 0b111;
            }
            2 => {
                // bst - combo modulo 8
                registers[1] = operand % 8;
            }
            3 => {
                // jnz - jump
                if registers[0] != 0 {
                    return compute(program, inst_position + 1, registers[0]);
                }
            }
            4 => {
                // bxc - bitwise XOR B C
                registers[1] ^= registers[2];
            }
            5 => {
                // out - combo modulo 8 output
                let check: u8 = (operand % 8) as u8;
                if check != program[inst_position] {
                    return false;
                }
                if inst_position == (program.len() - 1) {
                    return true;
                }
            }
            6 => {
                // bdv - adv stored B
                registers[1] = registers[0] >> operand;
            }
            7 => {
                // cdv - adv stored C
                registers[2] = registers[0] >> operand;
            }
            _ => unreachable!("{left} is invalid opcode"),
        };
    }

    false
}
fn get_operand(right: &u64, registers: &[u64; 3]) -> u64 {
    match right {
        0..=3 => *right,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!("Invalid operand {right}"),
    }
}

impl OpCodeProgram {
    fn run(&mut self, stop_length: Option<usize>) {
        let stop_length = stop_length.unwrap_or(9);
        let mut steps = 0;
        while !self.halt && self.output.len() <= stop_length && steps < 100 {
            self.compute();
            steps += 1;
        }
    }

    fn print_output(&self) -> String {
        self.output.iter().join(",")
    }

    fn get_operand(&self, right: &u8) -> u64 {
        match right {
            0..=3 => *right as u64,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            _ => unreachable!("Invalid operand {right}"),
        }
    }

    fn compute(&mut self) {
        let left = self.program.get(self.position);
        let right = self.program.get(self.position + 1);
        if left.is_none() || right.is_none() {
            self.halt = true;
            return;
        }

        let (left, right) = (left.unwrap(), right.unwrap());
        let operand = self.get_operand(right);

        let mut advance = true;

        match left {
            0 => {
                // adv - division
                self.registers.a >>= operand;
            }
            1 => {
                // bxl - bitwise XOR B and operand
                self.registers.b ^= *right as u64 & 0b111;
            }
            2 => {
                // bst - combo modulo 8
                self.registers.b = operand % 8;
            }
            3 => {
                // jnz - jump
                if self.registers.a != 0 {
                    self.position = *right as usize;
                    advance = false;
                }
            }
            4 => {
                // bxc - bitwise XOR B C
                self.registers.b ^= self.registers.c;
            }
            5 => {
                // out - combo modulo 8 output
                self.output.push(operand % 8);
            }
            6 => {
                // bdv - adv stored B
                self.registers.b = self.registers.a >> operand;
            }
            7 => {
                // cdv - adv stored C
                self.registers.c = self.registers.a >> operand;
            }
            _ => unreachable!("{left} is invalid opcode"),
        };

        if advance {
            self.position += 2;
        }
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| u64::from_str(s))(input)
}

fn parse_u8(input: &str) -> IResult<&str, u8> {
    map_res(digit1, |s: &str| u8::from_str(s))(input)
}

fn parse_register(input: &str) -> IResult<&str, (char, u64)> {
    let (input, _) = tag("Register ")(input)?;
    separated_pair(
        nom::character::complete::one_of("ABC"),
        tag(": "),
        parse_u64,
    )(input)
}

fn parse_registers(input: &str) -> IResult<&str, Registers> {
    let (input, _) = multispace0(input)?;
    let (input, (a_label, a_value)) = terminated(parse_register, tag("\n"))(input)?;
    let (input, (b_label, b_value)) = terminated(parse_register, tag("\n"))(input)?;
    let (input, (c_label, c_value)) = terminated(parse_register, tag("\n"))(input)?;

    assert_eq!(a_label, 'A');
    assert_eq!(b_label, 'B');
    assert_eq!(c_label, 'C');

    Ok((
        input,
        Registers {
            a: a_value,
            b: b_value,
            c: c_value,
        },
    ))
}

fn parse_program(input: &str) -> IResult<&str, Vec<u8>> {
    preceded(
        tag("Program: "),
        separated_list1(tag(","), preceded(multispace0, parse_u8)),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, OpCodeProgram> {
    let (input, registers) = parse_registers(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, program) = parse_program(input)?;

    Ok((
        input,
        OpCodeProgram {
            registers,
            program,
            output: Vec::new(),
            position: 0,
            halt: false,
        },
    ))
}

pub fn part_one(input: &str) -> Option<String> {
    let mut program = parse_input(input).unwrap().1;

    program.run(None);
    Some(program.print_output())
}

pub fn part_two(input: &str) -> Option<u64> {
    let program = parse_input(input).unwrap().1;
    let mut search: Vec<u64> = vec![0];

    // for each output, only the last three bits of "a" matter
    // find each combination that works as each step, then shift
    // them over by three bits to make room for the next test
    for target in (0..program.program.len()).rev() {
        // println!("\n");
        // search.iter().for_each(|s| println!("{:b}", s));
        let mut next_search_space: Vec<u64> = Vec::new();

        // 111 is 7 in binary, so to test each possible bitmask, we only need
        // to test 0->7
        // 0 -> 000
        // 1 -> 001
        // 2 -> 010
        // 3 -> 011
        // 4 -> 100
        // 5 -> 101
        // 6 -> 110
        // 7 -> 111
        let search_candidates = search.iter().flat_map(|a| (0..8).map(move |i| a + i));
        for a in search_candidates {
            if compute(&program.program, target, a) {
                if target == 0 {
                    return Some(a);
                }
                // only the lowest three bits matter for each output, so shift over
                next_search_space.push(a << 3);
            }
        }
        search = next_search_space;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
