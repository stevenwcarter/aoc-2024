advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut checksum: usize = 0;
    let mut index = 0;

    let input: Vec<usize> = input
        .trim_end()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect();

    let mut left_ptr = 0;
    let mut right_ptr = input.len() - 1;
    let mut needs_free_space = input[right_ptr];

    while left_ptr < right_ptr {
        for _ in 0..input[left_ptr] {
            checksum += (left_ptr / 2) * index;
            index += 1;
        }
        left_ptr += 1;

        for _ in 0..input[left_ptr] {
            if needs_free_space == 0 {
                right_ptr -= 2;
                if right_ptr <= left_ptr {
                    // stop once overlap starts
                    break;
                }
                needs_free_space = input[right_ptr];
            }
            checksum += (right_ptr / 2) * index;
            index += 1;
            needs_free_space -= 1;
        }
        left_ptr += 1;
    }

    for _ in 0..needs_free_space {
        checksum += (right_ptr / 2) * index;
        index += 1;
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input: Vec<usize> = input
        .trim_end()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect();

    let input_length = input.len();

    let mut checksum: usize = 0;

    let mut free_space_index = vec![0; input_length];
    for i in 1..input_length {
        free_space_index[i] = free_space_index[i - 1] + input[i - 1];
    }

    for right_ptr in (0..input_length).rev().step_by(2) {
        let mut free_space_found = false;

        for left_ptr in (1..right_ptr).step_by(2) {
            if input[left_ptr] >= input[right_ptr] {
                for i in 0..input[right_ptr] {
                    checksum += (right_ptr / 2) * (free_space_index[left_ptr] + i);
                }
                input[left_ptr] -= input[right_ptr];
                free_space_index[left_ptr] += input[right_ptr];
                free_space_found = true;
                break;
            }
        }

        if !free_space_found {
            for i in 0..input[right_ptr] {
                checksum += (right_ptr / 2) * (free_space_index[right_ptr] + i);
            }
        }
    }

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
