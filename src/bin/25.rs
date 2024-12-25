use hashbrown::HashMap;

advent_of_code::solution!(25);

#[derive(Debug, Clone)]
pub struct Data {
    pub keys: Vec<Vec<(u32, u32)>>,
    pub locks: Vec<Vec<(u32, u32)>>,
}

impl Data {
    pub fn parse_input(input: &str) -> Self {
        let mut keys: Vec<Vec<(u32, u32)>> = Vec::new();
        let mut locks: Vec<Vec<(u32, u32)>> = Vec::new();
        input.split("\n\n").for_each(|blob| {
            let lines = blob.lines().collect::<Vec<&str>>();
            let width = lines[0].chars().collect::<Vec<_>>().len();
            let is_lock = lines[0].chars().all(|c| c == '#');
            let mut heights: HashMap<u32, u32> = HashMap::new();
            let max = lines.len();
            lines.iter().for_each(|l| {
                l.chars().enumerate().for_each(|(idx, ch)| {
                    if ch == '#' {
                        *heights.entry(idx as u32).or_insert(0) += 1;
                    }
                });
            });

            let heights: Vec<(u32, u32)> = (0..width)
                .map(|idx| {
                    (
                        heights.entry(idx as u32).or_insert(0).saturating_sub(1),
                        max as u32,
                    )
                })
                .collect();

            if is_lock {
                locks.push(heights);
            } else {
                keys.push(heights);
            }
        });

        Self { keys, locks }
    }

    pub fn matches(key: &[(u32, u32)], lock: &[(u32, u32)]) -> bool {
        key.iter()
            .zip(lock)
            .all(|(key, lock)| key.0 + lock.0 <= key.1 - 2)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = Data::parse_input(input);

    let result = data
        .locks
        .iter()
        .map(|lock| {
            data.keys
                .iter()
                .filter(|key| Data::matches(key, lock))
                .count()
        })
        .sum();

    assert!(result != 24316);

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
