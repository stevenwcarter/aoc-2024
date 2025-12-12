advent_of_code::solution!(7);

use rayon::prelude::*;

fn parse_lines_iter(input: &str) -> impl ParallelIterator<Item = (u64, Vec<u64>)> + '_ {
    input.par_lines().filter_map(|line| {
        let (test, rest) = line.split_once(": ")?;
        let test_val = test.parse::<u64>().ok()?;

        let numbers = rest
            .split(' ')
            .map(|n| n.parse::<u64>().ok())
            .collect::<Option<Vec<_>>>()?;

        Some((test_val, numbers))
    })
}

fn find_operator_order(test_val: u64, nums: &[u64], idx: usize, current: u64, part2: bool) -> bool {
    if current > test_val {
        return false;
    }

    if idx == nums.len() {
        return current == test_val;
    }

    let next = nums[idx];

    // multiply
    if find_operator_order(test_val, nums, idx + 1, current * next, part2) {
        return true;
    }

    // add
    if find_operator_order(test_val, nums, idx + 1, current + next, part2) {
        return true;
    }

    // concatenate (part 2 only)
    if part2 {
        let concat = concatenate(current, next);
        if find_operator_order(test_val, nums, idx + 1, concat, part2) {
            return true;
        }
    }

    false
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
        parse_lines_iter(input)
            .filter(|(test, nums)| find_operator_order(*test, nums, 1, nums[0], false))
            .map(|(test, _)| test)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_lines_iter(input)
            .filter(|(test, nums)| find_operator_order(*test, nums, 1, nums[0], true))
            .map(|(test, _)| test)
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
