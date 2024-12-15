advent_of_code::solution!(7);

use rayon::prelude::*;
use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(
        parse_number,
        tag(": "),
        separated_list1(tag(" "), parse_number),
    )(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(multispace0, parse_line)(input)
}

fn find_operator_order(test_val: u64, numbers: &mut VecDeque<u64>, is_part_2: bool) -> Option<u64> {
    if numbers.len() == 1 {
        if *numbers.front().unwrap() == test_val {
            return Some(test_val);
        } else {
            return None;
        }
    }

    let first_number = numbers.pop_front().unwrap();
    let second_number = numbers.pop_front().unwrap();

    let check_result = |check_val: u64| {
        if check_val == test_val && numbers.is_empty() {
            return Some(test_val);
        }
        if check_val <= test_val {
            let mut check_val_vec = numbers.clone();
            check_val_vec.push_front(check_val);
            let check_val_result = find_operator_order(test_val, &mut check_val_vec, is_part_2);
            if check_val_result.is_some() {
                return check_val_result;
            }
        }
        None
    };

    let multiplied = check_result(first_number * second_number);
    if multiplied.is_some() {
        return multiplied;
    }

    let added = check_result(first_number + second_number);
    if added.is_some() {
        return added;
    }

    if is_part_2 {
        let concatenated = check_result(concatenate(first_number, second_number));
        if concatenated.is_some() {
            return concatenated;
        }
    }

    None
}

/// Counts the digits in the second number, then shifts the first number
/// over by the same amount and adds the second number where there are now zeroes
///
/// 123, 456 becomes
/// 123000 + 456 -> 123456
fn concatenate(a: u64, b: u64) -> u64 {
    let digits = b.ilog10() + 1;
    a * 10u64.pow(digits) + b
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_lines(input)
            .unwrap()
            .1
            .par_iter()
            .filter_map(|(test_val, numbers)| {
                let mut numbers = VecDeque::from(numbers.clone());
                find_operator_order(*test_val, &mut numbers, false)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_lines(input)
            .unwrap()
            .1
            .into_par_iter()
            .filter_map(|(test_val, numbers)| {
                let mut numbers = VecDeque::from(numbers);
                find_operator_order(test_val, &mut numbers, true)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3849));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11487));
    }
}
