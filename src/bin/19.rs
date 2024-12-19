use hashbrown::HashMap;
use rayon::prelude::*;
advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut iter = input.lines();
    let towels: Vec<&str> = iter.next().unwrap().split(", ").collect();
    iter.next();

    let mut patterns = Vec::new();
    for line in iter {
        patterns.push(line);
    }

    (towels, patterns)
}

fn has_valid_match(towels: &[&str], pattern: &str) -> bool {
    towels
        .iter()
        .filter(|t| pattern.starts_with(*t))
        .any(|prefix| pattern == *prefix || has_valid_match(towels, &pattern[prefix.len()..]))
}

fn count_valid_match(towels: &[&str], pattern: &str, cache: &mut HashMap<String, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(pattern) {
        return *count;
    }

    let count = towels
        .iter()
        .filter(|&&t| pattern.starts_with(t))
        .map(|t| count_valid_match(towels, &pattern[t.len()..], cache))
        .sum();

    cache.insert(pattern.to_string(), count);
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);

    Some(
        patterns
            .par_iter()
            .filter(|p| has_valid_match(&towels, p))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);

    Some(
        patterns
            .par_iter()
            .map(|pattern| count_valid_match(&towels, pattern, &mut HashMap::new()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
