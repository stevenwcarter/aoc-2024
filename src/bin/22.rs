advent_of_code::solution!(22);
use std::sync::{Arc, Mutex};

use hashbrown::HashMap;
use itertools::Itertools;

use rayon::prelude::*;

fn mix(n: u64, secret: u64) -> u64 {
    n ^ secret
}

fn prune(n: u64) -> u64 {
    n % 16777216
}
fn process(secret: u64, iters: usize) -> u64 {
    let mut n = secret;
    (0..iters).for_each(|_| {
        n = prune(mix(n * 64, n));
        n = prune(mix(n / 32, n));
        n = prune(mix(n * 2048, n));
    });

    n
}
fn changes(secret: u64, iters: usize) -> Vec<u8> {
    let mut n = secret;
    let mut vec: Vec<u8> = Vec::with_capacity(2001);
    vec.push((n % 10) as u8);
    vec.extend(
        (0..iters)
            .map(|_| {
                n = prune(mix(n * 64, n));
                n = prune(mix(n / 32, n));
                n = prune(mix(n * 2048, n));
                (n % 10) as u8
            })
            .collect::<Vec<u8>>(),
    );
    vec
}

fn build_key(a: u8, b: u8, c: u8, d: u8, e: u8) -> u32 {
    let d1 = b as i8 - a as i8;
    let d2 = c as i8 - b as i8;
    let d3 = d as i8 - c as i8;
    let d4 = e as i8 - d as i8;

    // construct a u32 key instead of using (i8,i8,i8,i8) as the key
    // use bitshifting to make room for each i8 so it remains unique
    let mut result: u32 = (d1 + 10) as u32;
    result <<= 5;
    result += (d2 + 10) as u32;
    result <<= 5;
    result += (d3 + 10) as u32;
    result <<= 5;
    result += (d4 + 10) as u32;
    result
}

fn find_best_iter(iters: &[Vec<u8>]) -> Option<u64> {
    let totals: Arc<Mutex<HashMap<u32, u64>>> = Arc::new(Mutex::new(HashMap::new()));

    iters.par_iter().for_each(|iter| {
        let mut inner_totals: HashMap<u32, u64> = HashMap::new();

        iter.iter()
            .tuple_windows()
            .for_each(|(&a, &b, &c, &d, &e)| {
                let key = build_key(a, b, c, d, e);

                if !inner_totals.contains_key(&key) {
                    inner_totals.entry(key).or_insert(e as u64);
                }
            });

        {
            let totals = totals.clone();
            let mut totals = totals.lock().unwrap();
            inner_totals.iter().for_each(|(k, v)| {
                *totals.entry(*k).or_insert(0) += *v;
            });
        }
    });

    let totals = totals.lock().unwrap();
    totals.iter().map(|(_, v)| v).copied().max()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|l| l.parse::<u64>().ok())
            .map(|n| process(n, 2000))
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u64> {
    let iter: Vec<Vec<u8>> = input
        .lines()
        .filter_map(|l| l.parse::<u64>().ok())
        .map(|n| changes(n, 2000))
        .collect();

    find_best_iter(&iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        assert_eq!(process(1, 2000), 8685429);
    }
    #[test]
    fn test_process_10() {
        assert_eq!(process(10, 2000), 4700978);
    }
    #[test]
    fn test_process_100() {
        assert_eq!(process(100, 2000), 15273692);
    }
    #[test]
    fn test_process_2024() {
        assert_eq!(process(2024, 2000), 8667524);
    }
    #[test]
    fn test_changes() {
        assert_eq!(changes(123, 9), vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
