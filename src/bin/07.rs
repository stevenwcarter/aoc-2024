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

    let added = first_number.checked_add(second_number);
    let multiplied = first_number.checked_mul(second_number);

    if let Some(added) = added {
        if added == test_val && numbers.is_empty() {
            return Some(test_val);
        }
        if added <= test_val {
            let mut added_vec = numbers.clone();
            added_vec.push_front(added);
            let added_result = find_operator_order(test_val, &mut added_vec, is_part_2);
            if added_result.is_some() {
                return added_result;
            }
        }
    }

    if let Some(multiplied) = multiplied {
        if multiplied == test_val && numbers.is_empty() {
            return Some(test_val);
        }
        if multiplied <= test_val {
            let mut multiplied_vec = numbers.clone();
            multiplied_vec.push_front(multiplied);
            let multiplied_result = find_operator_order(test_val, &mut multiplied_vec, is_part_2);
            if multiplied_result.is_some() {
                return multiplied_result;
            }
        }
    }

    if is_part_2 {
        let concatenated = concatenate(first_number, second_number);
        if let Some(concatenated) = concatenated {
            if concatenated == test_val && numbers.is_empty() {
                return Some(test_val);
            }
            if concatenated <= test_val {
                let mut concatenated_vec = numbers.clone();
                concatenated_vec.push_front(concatenated);
                let concatenated_result =
                    find_operator_order(test_val, &mut concatenated_vec, is_part_2);
                return concatenated_result;
            }
        }
    }

    None
}

fn concatenate(a: u64, b: u64) -> Option<u64> {
    let mut a = a.to_string();
    let b = b.to_string();
    a.push_str(&b);

    a.parse::<u64>().ok()
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
            .par_iter()
            .filter_map(|(test_val, numbers)| {
                let mut numbers = VecDeque::from(numbers.clone());
                find_operator_order(*test_val, &mut numbers, true)
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
