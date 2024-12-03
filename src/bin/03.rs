advent_of_code::solution!(3);

use regex::Regex;

fn find_and_sum_muls(input: &str) -> u32 {
    // Define the regex to match `mul(digits,digits)`
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;

    // Iterate over all matches
    for cap in re.captures_iter(input) {
        // Parse the captured groups as u32
        if let (Ok(a), Ok(b)) = (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
            total += a * b;
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_and_sum_muls(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
