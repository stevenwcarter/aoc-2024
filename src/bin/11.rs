use cached::proc_macro::cached;
use rayon::prelude::*;

advent_of_code::solution!(11);

pub fn split_stone(stone: u64) -> (u64, Option<u64>) {
    let stone = stone.to_string();
    let left = stone[0..(stone.len() / 2)].parse::<u64>().unwrap();
    let right = stone[(stone.len() / 2)..].parse::<u64>().unwrap();

    (left, Some(right))
}

pub fn is_even(stone: u64) -> bool {
    (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0
}

pub fn process_stone(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }
    if is_even(stone) {
        return split_stone(stone);
    }
    (stone * 2024, None)
}

#[cached]
pub fn count_stones(stone: u64, count: u8) -> u64 {
    let mut found_count: u64 = 1;
    let mut current = stone;

    (0..count).rev().for_each(|c| {
        let (updated, right) = process_stone(current);
        current = updated;
        if let Some(right) = right {
            found_count += count_stones(right, c);
        }
    });

    found_count
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .trim_end()
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    Some(stones.iter().map(|n| count_stones(*n, 25)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .trim_end()
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    Some(stones.par_iter().map(|n| count_stones(*n, 75)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let result = count_stones(10, 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
