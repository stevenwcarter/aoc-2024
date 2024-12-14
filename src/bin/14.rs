advent_of_code::solution!(14);

use rayon::prelude::*;
use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::available_parallelism,
};

use hashbrown::HashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i64, multispace0, multispace1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

type RobotPosition = (usize, usize);
type RobotVelocity = (i64, i64);
type RobotPositionsAndVelocity = (RobotPosition, RobotVelocity);

fn parse_line(input: &str) -> IResult<&str, RobotPositionsAndVelocity> {
    let parse_usize_pair = map(separated_pair(u64, char(','), u64), |(a, b)| {
        (a as usize, b as usize)
    });
    let parse_i64_pair = separated_pair(i64, char(','), i64);

    let parse_p = preceded(tag("p="), parse_usize_pair);
    let parse_v = preceded(tag("v="), parse_i64_pair);

    map(tuple((parse_p, multispace1, parse_v)), |(p, _, v)| (p, v))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<RobotPositionsAndVelocity>> {
    separated_list1(multispace0, parse_line)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let steps: i64 = 100;
    let is_test = input.len() < 200;
    let width = if is_test { 11 } else { 101 };
    let height = if is_test { 7 } else { 103 };
    let robots = parse_input(input).unwrap().1;

    let updated_robots: Vec<(usize, usize)> = robots
        .iter()
        .map(|robot| step_robot(robot, steps, width, height))
        .collect();

    let half_x = width / 2;
    let half_y = height / 2;
    let mut quadrants: HashMap<u8, u32> = HashMap::new();
    updated_robots
        .iter()
        .filter_map(|(px, py)| {
            if px < &half_x && py < &half_y {
                Some(1)
            } else if px > &half_x && py < &half_y {
                Some(2)
            } else if px < &half_x && py > &half_y {
                Some(3)
            } else if px > &half_x && py > &half_y {
                Some(4)
            } else {
                None
            }
        })
        .for_each(|q| {
            *quadrants.entry(q).or_insert(0) += 1;
        });

    Some(quadrants.iter().map(|(_, v)| v).product())
}

fn step_robot(
    ((px, py), (vx, vy)): &((usize, usize), (i64, i64)),
    steps: i64,
    width: usize,
    height: usize,
) -> (usize, usize) {
    let px = (*px as i64 + vx * steps).rem_euclid(width as i64) as usize;
    let py = (*py as i64 + vy * steps).rem_euclid(height as i64) as usize;

    (px, py)
}

fn find_lines(points: &mut [(usize, usize)]) -> bool {
    // Sort points by (x, y)
    points.sort_unstable();

    let mut last_x = None;
    let mut last_y = None;
    let mut count = 0;

    let quarter_idx = points.len() / 4;
    let three_quarter_idx = quarter_idx * 3;

    for &mut (x, y) in &mut points[quarter_idx..three_quarter_idx] {
        if Some(x) == last_x && Some(y) == last_y.map(|ly| ly + 1) {
            count += 1;
            if count >= 10 {
                return true;
            }
        } else {
            count = 1;
        }
        last_x = Some(x);
        last_y = Some(y);
    }

    false
}

pub fn part_two(input: &str) -> Option<usize> {
    let is_test = input.len() < 200;
    let width = if is_test { 11 } else { 101 };
    let height = if is_test { 7 } else { 103 };
    let robots = parse_input(input).unwrap().1;

    let thread_count = available_parallelism().unwrap().get().max(16);
    let found = AtomicBool::new(false);
    let steps = AtomicUsize::new(0);

    (0..thread_count).into_par_iter().for_each(|offset| {
        let mut i = 0;
        loop {
            let check = i * thread_count + offset;
            if check > 10000 || found.load(Ordering::Relaxed) {
                return;
            }
            let mut updated_robots: Vec<(usize, usize)> = robots
                .iter()
                .map(|robot| step_robot(robot, check as i64, width, height))
                .collect();
            if find_lines(&mut updated_robots) {
                found.store(true, Ordering::Relaxed);
                steps.store(check, Ordering::Relaxed);
            } else {
                i += 1;
            }
        }
    });

    Some(steps.load(Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
