advent_of_code::solution!(8);

use aoc_mine::Coord;
use hashbrown::HashSet;
use rayon::prelude::*;

pub fn part_one(input: &str) -> Option<usize> {
    let (row_count, antennas) = parse_antennas(input);

    let antinodes = (0..input.lines().next().unwrap().len())
        .into_par_iter()
        .fold(HashSet::new, |mut acc, x| {
            for y in 0..row_count {
                for i in 0..antennas.len() {
                    for j in 0..antennas.len() {
                        if i == j {
                            continue;
                        }
                        let (coord1, freq1) = antennas[i];
                        let (coord2, freq2) = antennas[j];

                        if freq1 != freq2 {
                            continue;
                        }

                        let dx1 = (x as isize - coord1.x()).abs();
                        let dy1 = (y as isize - coord1.y()).abs();
                        let dx2 = (x as isize - coord2.x()).abs();
                        let dy2 = (y as isize - coord2.y()).abs();

                        let points: Vec<Coord<isize>> =
                            vec![(x as isize, y as isize).into(), coord1, coord2];

                        // Ensure the second antenna is twice as far from the antinode
                        if Coord::points_are_linear(&points[..])
                            && (dx1 == 2 * dx2 || dx2 == 2 * dx1)
                            && (dy1 == 2 * dy2 || dy2 == 2 * dy1)
                        {
                            acc.insert((x, y));
                        }
                    }
                }
            }
            acc
        })
        .reduce_with(|mut m1, m2| {
            for k in m2 {
                m1.insert(k);
            }
            m1
        })
        .unwrap();

    Some(antinodes.len())
}

fn parse_antennas(input: &str) -> (usize, Vec<(Coord<isize>, char)>) {
    let mut antennas: Vec<(Coord<isize>, char)> = Vec::new();

    let mut row_count = 0;
    for (y, row) in input.lines().enumerate() {
        row_count += 1;
        for (x, ch) in row.chars().enumerate() {
            if ch.is_alphanumeric() {
                antennas.push(((x as isize, y as isize).into(), ch));
            }
        }
    }

    (row_count, antennas)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (row_count, antennas) = parse_antennas(input);

    let antinodes = (0..input.lines().next().unwrap().len())
        .into_par_iter()
        .fold(HashSet::new, |mut acc, x| {
            for y in 0..row_count {
                for i in 0..antennas.len() {
                    for j in 0..antennas.len() {
                        if i == j {
                            continue;
                        }
                        let (coord1, freq1) = antennas[i];
                        let (coord2, freq2) = antennas[j];

                        if freq1 != freq2 {
                            continue;
                        }

                        let points: Vec<Coord<isize>> =
                            vec![(x as isize, y as isize).into(), coord1, coord2];

                        // Ensure the second antenna is twice as far from the antinode
                        if Coord::points_are_linear(&points[..]) {
                            acc.insert((x, y));
                        }
                    }
                }
            }
            acc
        })
        .reduce_with(|mut m1, m2| {
            for k in m2 {
                m1.insert(k);
            }
            m1
        })
        .unwrap();

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
