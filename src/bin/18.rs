advent_of_code::solution!(18);

use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(',').collect();
            (
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn find_shortest_path(byte_positions: &[(usize, usize)], blocks: usize) -> Option<usize> {
    let mut corrupted = HashSet::new();
    let grid_size = if byte_positions.len() > 100 { 71 } else { 7 };

    for &(x, y) in byte_positions.iter().take(blocks) {
        corrupted.insert((x, y));
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, 0, 0)); // (x, y, steps)
    visited.insert((0, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if (x, y) == (grid_size - 1, grid_size - 1) {
            return Some(steps);
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < grid_size as isize && ny < grid_size as isize {
                let next = (nx as usize, ny as usize);

                if !corrupted.contains(&next) && !visited.contains(&next) {
                    queue.push_back((next.0, next.1, steps + 1));
                    visited.insert(next);
                }
            }
        }
    }

    None
}

pub fn start_offset_test_vs_prod(input_length: usize) -> usize {
    if input_length > 100 {
        1024
    } else {
        12
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let byte_positions = parse_input(input);

    let start_offset = start_offset_test_vs_prod(byte_positions.len());

    find_shortest_path(&byte_positions, start_offset)
}

pub fn part_two(input: &str) -> Option<String> {
    let byte_positions = parse_input(input);

    let start_offset = if byte_positions.len() > 100 { 1024 } else { 12 };

    let indexes: Vec<usize> = (start_offset..byte_positions.len()).collect();

    let byte_pos_idx = indexes
        .binary_search_by(|p| {
            let index = p;
            let first_result = find_shortest_path(&byte_positions, *index);
            if first_result.is_none() {
                let before = find_shortest_path(&byte_positions, index - 1);
                if before.is_some() {
                    // this is the iteration we want
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Less
            }
        })
        .unwrap();

    // I offset this by 1024 to limit the search space, now it needs to be added back
    // to find the actual index.
    let byte_pos = byte_positions[byte_pos_idx + start_offset - 1];

    Some(format!("{},{}", byte_pos.0, byte_pos.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
