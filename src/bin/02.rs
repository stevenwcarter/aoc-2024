use std::cmp;

use nom::{
    character::complete::{space1, u32},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(2);

pub fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(input)
}

fn absolute_diff(num1: u32, num2: u32) -> u32 {
    cmp::max(num1, num2) - cmp::min(num1, num2)
}

fn is_difference_in_range(data: &[u32]) -> bool {
    data.windows(2).all(|w| {
        let a = absolute_diff(w[0], w[1]);
        a > 0 && a < 4
    })
}
fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] < w[1])
}
fn is_sorted_rev<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[1] < w[0])
}

pub fn is_safe(vec: &[u32]) -> bool {
    (is_sorted(vec) || is_sorted_rev(vec)) && is_difference_in_range(vec)
}
pub fn is_safe_with_dampening(vec: &[u32]) -> bool {
    if is_safe(vec) {
        return true;
    }
    for index in 0..vec.len() {
        let l = slice_without_nth(vec, index);
        if is_safe(&l) {
            return true;
        }
    }

    false
}

fn slice_without_nth(vec: &[u32], n: usize) -> Vec<u32> {
    let len = vec.len();
    if n >= len {
        panic!("Index out of bounds");
    }

    // Create two slices: one before and one after the nth element
    let before = &vec[..n];
    let after = &vec[n + 1..];

    // Concatenate the two slices into a single slice
    [before, after].concat()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u32>> = input.lines().map(|l| parse_line(l).unwrap().1).collect();

    Some(lines.iter().filter(|l| is_safe(l)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u32>> = input.lines().map(|l| parse_line(l).unwrap().1).collect();

    Some(lines.iter().filter(|l| is_safe_with_dampening(l)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
