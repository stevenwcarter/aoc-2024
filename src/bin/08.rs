advent_of_code::solution!(8);

use aoc_mine::Coord;
use hashbrown::HashSet;
use rayon::prelude::*;

use hashbrown::HashMap;

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

fn parse_antennas(input: &str) -> (isize, isize, HashMap<char, Vec<Coord<isize>>>) {
    let mut map: HashMap<char, Vec<Coord<isize>>> = HashMap::new();
    let mut height = 0;
    let mut width = 0;

    for (y, row) in input.lines().enumerate() {
        height += 1;
        width = row.len();
        for (x, ch) in row.chars().enumerate() {
            if ch.is_alphanumeric() {
                map.entry(ch)
                    .or_default()
                    .push((x as isize, y as isize).into());
            }
        }
    }

    (width as isize, height as isize, map)
}

#[inline]
fn in_bounds(p: Coord<isize>, w: isize, h: isize) -> bool {
    p.x() >= 0 && p.y() >= 0 && p.x() < w && p.y() < h
}

pub fn part_one(input: &str) -> Option<usize> {
    let (w, h, groups) = parse_antennas(input);

    let antinodes: HashSet<(isize, isize)> = groups
        .par_values()
        .map(|antennas| {
            let mut local = HashSet::new();

            for i in 0..antennas.len() {
                for j in i + 1..antennas.len() {
                    let a = antennas[i];
                    let b = antennas[j];

                    let p1 = (2 * b.x() - a.x(), 2 * b.y() - a.y()).into();
                    let p2 = (2 * a.x() - b.x(), 2 * a.y() - b.y()).into();

                    if in_bounds(p1, w, h) {
                        local.insert((p1.x(), p1.y()));
                    }
                    if in_bounds(p2, w, h) {
                        local.insert((p2.x(), p2.y()));
                    }
                }
            }
            local
        })
        .reduce(HashSet::new, |mut a, b| {
            a.extend(b);
            a
        });

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (w, h, groups) = parse_antennas(input);

    let antinodes: HashSet<(isize, isize)> = groups
        .par_values()
        .map(|antennas| {
            let mut local = HashSet::new();

            for i in 0..antennas.len() {
                for j in i + 1..antennas.len() {
                    let a = antennas[i];
                    let b = antennas[j];

                    let dx = b.x() - a.x();
                    let dy = b.y() - a.y();
                    let g = gcd(dx, dy);

                    let step = (dx / g, dy / g);

                    // walk forward
                    let mut p = a;
                    while in_bounds(p, w, h) {
                        local.insert((p.x(), p.y()));
                        p = (p.x() + step.0, p.y() + step.1).into();
                    }

                    // walk backward
                    let mut p = a;
                    while in_bounds(p, w, h) {
                        local.insert((p.x(), p.y()));
                        p = (p.x() - step.0, p.y() - step.1).into();
                    }
                }
            }
            local
        })
        .reduce(HashSet::new, |mut a, b| {
            a.extend(b);
            a
        });

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
