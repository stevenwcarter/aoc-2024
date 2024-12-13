advent_of_code::solution!(5);

use rayon::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{cmp::Ordering, str::FromStr};

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| u32::from_str(s))(input)
}

fn parse_pipe_line(input: &str) -> IResult<&str, (u32, u32)> {
    terminated(
        separated_pair(parse_number, tag("|"), parse_number),
        line_ending,
    )(input)
}

fn parse_comma_line(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(separated_list1(tag(","), parse_number), line_ending)(input)
}

type Rules = Vec<(u32, u32)>;
type Data = Vec<Vec<u32>>;

fn parse_input(input: &str) -> IResult<&str, (Rules, Data)> {
    let (input, pipe_lines) = many1(parse_pipe_line)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, comma_lines) = many1(parse_comma_line)(input)?;
    Ok((input, (pipe_lines, comma_lines)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (checks, data) = parse_input(input).unwrap().1;

    Some(
        data.par_iter()
            .filter(|d| d.is_sorted_by(|x, y| !checks.contains(&(*y, *x))))
            .map(|d| d[d.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (checks, mut data) = parse_input(input).unwrap().1;

    let comparator = |x: &u32, y: &u32| {
        if checks.contains(&(*y, *x)) {
            Ordering::Greater
        } else if checks.contains(&(*x, *y)) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };

    Some(
        data.par_iter_mut()
            .filter(|d| !d.is_sorted_by(|a, b| comparator(a, b) != Ordering::Greater))
            .map(|d| {
                d.sort_by(comparator);
                d
            })
            .map(|d| d[d.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
