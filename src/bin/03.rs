advent_of_code::solution!(3);

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_mul(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("mul(")(input)?; // Must start with `mul(`
    let (input, (a, b)) = separated_pair(parse_number, char(','), parse_number)(input)?;
    let (input, _) = char(')')(input)?; // Must end with `)`
    Ok((input, a * b))
}

fn parse_do(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, ()))
}

fn parse_dont(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, ()))
}

fn process_instructions_1(input: &str, is_part_1: bool) -> u32 {
    let mut input = input;
    let mut total = 0;

    while !input.is_empty() {
        if let Ok((remaining, result)) = parse_mul(input) {
            total += result;
            input = remaining;
            continue;
        }

        input = &input[1..];
    }

    total
}
fn process_instructions(input: &str, is_part_1: bool) -> u32 {
    let mut input = input;
    let mut total = 0;
    let mut mul_enabled = true; // At the start, `mul` instructions are enabled

    while !input.is_empty() {
        if !is_part_1 {
            if let Ok((remaining, _)) = parse_do(input) {
                mul_enabled = true;
                input = remaining;
                continue;
            }

            if let Ok((remaining, _)) = parse_dont(input) {
                mul_enabled = false;
                input = remaining;
                continue;
            }
        }

        if let Ok((remaining, result)) = parse_mul(input) {
            if mul_enabled {
                total += result;
            }
            input = remaining;
            continue;
        }

        input = &input[1..];
    }

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(process_instructions(input, true))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(process_instructions(input, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
