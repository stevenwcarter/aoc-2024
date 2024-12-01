use hashbrown::HashMap;
use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| match parse_data_nom(l) {
            Ok((_, (a, b))) => (a, b),
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!("Error unwrapping");
            }
        })
        .unzip();
    a.sort();
    b.sort();

    Some(
        (0..a.len())
            .map(|i| {
                let a = a.get(i).unwrap();
                let b = b.get(i).unwrap();
                if a > b {
                    a - b
                } else {
                    b - a
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| match parse_data_nom(l) {
            Ok((_, (a, b))) => (a, b),
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!("Error unwrapping");
            }
        })
        .unzip();
    let mut b_counts: HashMap<u32, u32> = HashMap::new();
    for n in b {
        *b_counts.entry(n).or_insert(0) += 1;
    }

    Some(
        a.into_iter()
            .map(|e| {
                let b_frequency = *b_counts.entry(e).or_default();
                e * b_frequency
            })
            .sum(),
    )
}

fn number_parser(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_data_nom(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(number_parser, space1, number_parser)(input)
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
