advent_of_code::solution!(13);

use nom::{
    bytes::complete::tag,
    character::complete::i64,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

const PART_2_OFFSET: i64 = 10_000_000_000_000;

type Coord = (i64, i64);
fn parse_input(input: &str) -> IResult<&str, Vec<(Coord, Coord, Coord)>> {
    let machine = tuple((
        preceded(tag("Button A: X+"), separated_pair(i64, tag(", Y+"), i64)),
        preceded(tag("\nButton B: X+"), separated_pair(i64, tag(", Y+"), i64)),
        preceded(tag("\nPrize: X="), separated_pair(i64, tag(", Y="), i64)),
    ));
    separated_list1(tag("\n\n"), machine)(input)
}

// originally solved part 1 with an iterative approach.. that certainly
// didn't work for part 2. Algebra to the rescue!
fn solve_machine(a: Coord, b: Coord, p: Coord) -> Option<i64> {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (px, py) = p;

    let denominator = bx * ay - ax * by;
    if denominator == 0 {
        // can't divide by zero, so no valid solution
        return None;
    }

    let numerator = bx * py - by * px;
    if numerator % denominator != 0 {
        // equations don't align to discrete button presses
        return None;
    }

    let x = numerator / denominator;
    let y = (px - ax * x) / bx;

    if ay * x + by * y != py {
        // button presses for x solution don't line up on the y-axis
        return None;
    }

    Some(x * 3 + y)
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        parse_input(input)
            .unwrap()
            .1
            .into_iter()
            .filter_map(|(a, b, p)| solve_machine(a, b, p))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        parse_input(input)
            .unwrap()
            .1
            .into_iter()
            .map(|(a, b, p)| {
                let (mut px, mut py) = p;
                px += PART_2_OFFSET;
                py += PART_2_OFFSET;

                (a, b, (px, py))
            })
            .filter_map(|(a, b, p)| solve_machine(a, b, p))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
