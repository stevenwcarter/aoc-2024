advent_of_code::solution!(22);

use atoi_simd::parse;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use nohash::BuildNoHashHasher;
use rayon::prelude::*;

const ITERATIONS: usize = 2000;

#[cfg(not(target_env = "msvc"))]
#[cfg(not(feature = "dhat"))]
use jemallocator::Jemalloc;

#[cfg(not(feature = "dhat"))]
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

/// bitwise XOR of new number and the original secret
#[inline]
fn mix(n: u64, secret: u64) -> u64 {
    n ^ secret
}

/// Determine modulo of secret to keep it from growing too large
#[inline]
fn prune(n: u64) -> u64 {
    n % 16777216
}

/// Calculate the next secret number based on the previous one,
/// and do it `iters` times.
fn process(secret: u64, iters: usize) -> u64 {
    let mut n = secret;
    for _ in 0..iters {
        next_iter(&mut n);
    }

    n
}

#[inline]
fn next_iter(n: &mut u64) {
    *n = prune(mix(*n * 64, *n));
    *n = prune(mix(*n / 32, *n));
    *n = prune(mix(*n * 2048, *n));
}

/// Given the secret and the number of iterations, return all the
/// prices for all 2000 iterations as an iterator to reduce allocations
#[inline]
fn changes(secret: u64, iters: usize) -> impl Iterator<Item = u8> {
    let mut n = secret;
    std::iter::once((n % 10) as u8).chain((0..iters).map(move |_| {
        next_iter(&mut n);
        (n % 10) as u8
    }))
}

/// Builds a key for the HashMap given the different prices. Calculate the differences,
/// and use them to build the key. Squash them all into a u32
fn build_key(a: u8, b: u8, c: u8, d: u8, e: u8) -> u32 {
    let d1 = ((b as i8 - a as i8) + 10) as u8;
    let d2 = ((c as i8 - b as i8) + 10) as u8;
    let d3 = ((d as i8 - c as i8) + 10) as u8;
    let d4 = ((e as i8 - d as i8) + 10) as u8;

    u32::from_ne_bytes([d1, d2, d3, d4])
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|l| parse::<u64>(l.as_bytes()).ok())
            .map(|n| process(n, ITERATIONS))
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    // let totals: DashMap<u32, u32> = DashMap::new();
    let totals = input
        .lines()
        .par_bridge()
        .filter_map(|l| parse::<u64>(l.as_bytes()).ok())
        .map(|n| changes(n, ITERATIONS))
        .fold(HashMap::<u32, u32>::new, |mut totals, iter| {
            let mut seen: HashSet<u32, BuildNoHashHasher<u32>> =
                HashSet::with_capacity_and_hasher(2001, BuildNoHashHasher::default());

            iter.tuple_windows()
                .map(|(a, b, c, d, e)| (build_key(a, b, c, d, e), e))
                .filter(|(key, _)| seen.insert(*key))
                .for_each(|(key, e)| {
                    *totals.entry(key).or_insert(0) += e as u32;
                });

            totals
        })
        .reduce(HashMap::<u32, u32>::new, |mut a, b| {
            b.iter().for_each(|(k, v)| {
                *a.entry(*k).or_insert(0) += v;
            });
            a
        });

    totals.values().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        assert_eq!(process(1, ITERATIONS), 8685429);
    }
    #[test]
    fn test_process_10() {
        assert_eq!(process(10, ITERATIONS), 4700978);
    }
    #[test]
    fn test_process_100() {
        assert_eq!(process(100, ITERATIONS), 15273692);
    }
    #[test]
    fn test_process_2024() {
        assert_eq!(process(2024, ITERATIONS), 8667524);
    }
    #[test]
    fn test_changes() {
        assert_eq!(
            changes(123, 9).collect::<Vec<u8>>(),
            vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]
        );
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
