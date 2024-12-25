advent_of_code::solution!(24);

use std::collections::VecDeque;

use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationType {
    XOR,
    OR,
    AND,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operation<'a> {
    pub left: &'a str,
    pub right: &'a str,
    pub output: &'a str,
    pub op_type: OperationType,
}

#[derive(Debug, Clone, Default)]
pub struct Data<'a> {
    pub values: HashMap<&'a str, bool>,
    pub operations: Vec<Operation<'a>>,
    pub operation_map: HashMap<&'a str, Operation<'a>>,
}

impl<'a> Data<'a> {
    fn parse_input_with_swaps(input: &'a str, needed_swaps: &[(&'a str, &'a str)]) -> Self {
        let (initials, operations) = input.split_once("\n\n").unwrap();

        let mut swaps: HashMap<&str, &str> = HashMap::new();

        needed_swaps.iter().for_each(|(a, b)| {
            swaps.insert(a, b);
            swaps.insert(b, a);
        });

        let values: HashMap<&str, bool> = initials
            .lines()
            .map(|l| {
                let (id, v) = l.split_once(": ").unwrap();
                let v = match v {
                    "1" => true,
                    "0" => false,
                    _ => unreachable!("invalid value {v}"),
                };

                (id, v)
            })
            .collect();

        let operations: Vec<Operation> = operations
            .lines()
            .map(|l| {
                let (inputs, mut output) = l.split_once(" -> ").unwrap();

                // swaps the output with its identified partner
                if let Some(n) = swaps.get(output) {
                    output = n;
                }

                let inputs: Vec<&str> = inputs.split_whitespace().collect();

                let (left, op, right) = (inputs[0], inputs[1], inputs[2]);

                let op_type = match op {
                    "AND" => OperationType::AND,
                    "OR" => OperationType::OR,
                    "XOR" => OperationType::XOR,
                    _ => unreachable!("Unknown operations {op}"),
                };
                Operation {
                    left,
                    right,
                    output,
                    op_type,
                }
            })
            .collect();

        // Build a map to make it easier to look up operations for the given output
        let operation_map: HashMap<&str, Operation> =
            operations.iter().map(|o| (o.output, *o)).collect();

        Self {
            values,
            operations,
            operation_map,
        }
    }

    fn solve(&mut self) {
        let targets: Vec<&str> = self
            .operations
            .iter()
            .map(|o| o.output)
            .filter(|o| o.starts_with("z"))
            .filter(|o| o[1..].parse::<u8>().is_ok())
            .collect();

        for target in targets {
            if self.values.contains_key(target) {
                // already solved
                continue;
            }
            let _ = self.get_value(target);
        }
    }

    fn get_value(&mut self, value: &str) -> Option<bool> {
        if let Some(value) = self.values.get(value) {
            return Some(*value);
        }
        let operation = *self.operation_map.get(value).expect("No mapping for value");
        let Operation {
            left,
            right,
            output,
            op_type,
        } = operation;
        let left = self.get_value(left);
        let right = self.get_value(right);
        match (left, right) {
            (Some(left), Some(right)) => {
                let result = match op_type {
                    OperationType::XOR => left != right,
                    OperationType::OR => left || right,
                    OperationType::AND => left && right,
                };
                self.values.insert(output, result);
                Some(result)
            }
            _ => None,
        }
    }

    /// Extract the value in z
    pub fn value_z(&self) -> Option<u64> {
        let mut targets: Vec<u8> = self
            .operations
            .iter()
            .map(|o| o.output)
            .filter(|o| o.starts_with("z"))
            .filter_map(|o| o[1..].parse::<u8>().ok())
            .collect();
        targets.sort();
        let binary = targets
            .iter()
            .rev()
            .map(|n| {
                self.values
                    .get(format!("z{:0>2}", n).as_str())
                    .expect("Unknown index")
            })
            .map(|&v| match v {
                true => '1',
                false => '0',
            })
            .join("");

        u64::from_str_radix(binary.as_str(), 2).ok()
    }

    /// Override the y wires with a u64 input
    pub fn set_ys(&mut self, value: u64) {
        (0..45).for_each(|idx| {
            self.set_wire('y', idx, get_bit_at(value, idx));
        });
    }

    /// Override the x wires with a u64 input
    pub fn set_xs(&mut self, value: u64) {
        (0..45).for_each(|idx| {
            self.set_wire('x', idx, get_bit_at(value, idx));
        });
    }

    /// used to override the starting condition for a wire
    fn set_wire(&mut self, prefix: char, n: u8, value: bool) {
        let key = format!("{prefix}{:0>2}", n);
        let (_, v) = self
            .values
            .iter_mut()
            .find(|(&k, _)| k == key.as_str())
            .unwrap();
        *v = value;
    }

    /// Find all wires that impact the provided output
    pub fn find_affecting_wires(&self, target: &str) -> Vec<&str> {
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_front(target);

        // Used a LinkedHashSet to preserve order when I was looking at the
        // output graph for affected wires. Didn't end up using this, but it
        // was helpful to visualize how the progression worked (2 inputs for 0,
        // 6 for 2, 12 for 3, etc.)
        let mut seen: Vec<&str> = Vec::new();

        while !queue.is_empty() {
            let target = queue.pop_front().unwrap();
            if let Some(o) = self.operation_map.get(target) {
                let Operation {
                    left,
                    right,
                    output: _,
                    op_type: _,
                } = o;
                let mut s: Vec<&str> = Vec::new();
                seen.push(left);
                seen.push(right);
                // if seen.insert(left) {
                s.push(left);
                // }
                // if seen.insert(right) {
                s.push(right);
                // }
                s.sort();
                s.iter().for_each(|v| {
                    queue.push_back(v);
                });
            };
        }

        seen.into_iter().collect::<Vec<&str>>()
    }
}

fn get_bit_at(input: u64, n: u8) -> bool {
    if n > 45 {
        unreachable!("out of range for this problem");
    }
    input & (1 << n) != 0
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut data = Data::parse_input_with_swaps(input, &[]);

    // println!("{:#?}", data);
    data.solve();
    // println!("{:#?}", data);
    data.value_z()
}

// Used this diagram https://content.instructables.com/FMU/D6P1/LJ1FVETK/FMUD6P1LJ1FVETK.jpg?auto=webp&frame=1&fit=bounds&md=MjAyMy0wNi0xOCAxMzo1MDoyMy4w
// Ran this code (without the swaps the first time) to find when bits started being wrong, then
// started from n-1 to trace the AND/OR for the carry bit, along with the trace for n. The errors
// were pretty obvious once you started going through.
pub fn part_two(input: &str) -> Option<String> {
    // As swaps were identified, I put them here
    let swaps = vec![
        ("z08", "thm"),
        ("wrm", "wss"),
        ("hwq", "z22"),
        ("z29", "gbs"),
    ];
    let source_data = Data::parse_input_with_swaps(input, &swaps);

    // create new test values for x/y since the circuit should do x+y=z
    // This made it easy to see when something was wrong
    let xs: u64 = 1; // all zeros
    let ys: u64 = 2u64.pow(45) - 2; // 44 ones preceded by zeros, final output should match this

    let mut data = source_data.clone();
    data.set_ys(ys);
    data.set_xs(xs);
    data.solve();
    let result = data.value_z().unwrap();

    // Keep track of bits which don't match their expected result. I did this
    // and then began looking at the first wrong number as n, as well as n-1 since
    // the carry bit from the previous operation could be wrong as well
    let mut bad_bits: Vec<u8> = Vec::new();
    (0..45).for_each(|idx| {
        let y_bit = get_bit_at(ys + xs, idx);
        let z_bit = get_bit_at(result, idx);

        if y_bit != z_bit {
            bad_bits.push(idx);
        }
    });

    if !bad_bits.is_empty() {
        println!("Bad bits: {:#?}", bad_bits);
    }

    // (0..3).for_each(|idx| {
    //     let key = format!("z{:0>2}", idx);
    //     let wires = data.find_affecting_wires(&key);
    //     println!("Wires affecting {key}: {:#?}", wires);
    // });

    // once the swaps were found, update this structure to give me the
    // alphabetical sorted list joined by commas
    let mut swaps = swaps.iter().fold(Vec::new(), |mut array, c| {
        array.push(c.0);
        array.push(c.1);
        array
    });
    swaps.sort();
    Some(swaps.iter().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some("gbs,hwq,thm,wrm,wss,z08,z22,z29".to_string()));
    }
}
