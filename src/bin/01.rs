use hashbrown::HashMap;
use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b): (Vec<_>, Vec<_>) = parse_input(input).unwrap().1.iter().copied().unzip();
    a.sort();
    b.sort();

    Some(a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b): (Vec<_>, Vec<_>) = parse_input(input).unwrap().1.iter().copied().unzip();

    let mut b_counts: HashMap<u32, u32> = HashMap::with_capacity(1000);
    for n in b {
        *b_counts.entry(n).or_insert(0) += 1;
    }

    Some(a.iter().map(|e| e * b_counts.get(e).unwrap_or(&0)).sum())
}

fn number_parser(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_data_nom(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(number_parser, space1, number_parser)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many1(terminated(parse_data_nom, newline))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
