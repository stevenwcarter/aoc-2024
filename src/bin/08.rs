advent_of_code::solution!(8);

use hashbrown::HashSet;

fn points_are_linear(points: &[(isize, isize)]) -> bool {
    // Fewer than 3 points are always collinear
    if points.len() < 3 {
        return true;
    }

    let (x1, y1) = points[0];
    let (x2, y2) = points[1];
    let ref_dx = x2 - x1;
    let ref_dy = y2 - y1;

    for &(x, y) in &points[2..] {
        let dx = x - x1;
        let dy = y - y1;

        if ref_dx * dy != ref_dy * dx {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut antennas: Vec<(usize, usize, char)> = Vec::new();

    let mut row_count = 0;
    for (y, row) in input.lines().enumerate() {
        row_count += 1;
        for (x, ch) in row.chars().enumerate() {
            if ch.is_alphanumeric() {
                antennas.push((x, y, ch));
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for x in 0..input.lines().next().unwrap().len() {
        for y in 0..row_count {
            for i in 0..antennas.len() {
                for j in 0..antennas.len() {
                    if i == j {
                        continue;
                    }
                    let (x1, y1, freq1) = antennas[i];
                    let (x2, y2, freq2) = antennas[j];

                    if freq1 != freq2 {
                        continue;
                    }

                    let dx1 = (x as isize - x1 as isize).abs();
                    let dy1 = (y as isize - y1 as isize).abs();
                    let dx2 = (x as isize - x2 as isize).abs();
                    let dy2 = (y as isize - y2 as isize).abs();

                    let points: Vec<(isize, isize)> = vec![
                        (x as isize, y as isize),
                        (x1 as isize, y1 as isize),
                        (x2 as isize, y2 as isize),
                    ];

                    // Ensure the second antenna is twice as far from the antinode
                    if points_are_linear(&points[..])
                        && (dx1 == 2 * dx2 || dx2 == 2 * dx1)
                        && (dy1 == 2 * dy2 || dy2 == 2 * dy1)
                    {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut antennas: Vec<(usize, usize, char)> = Vec::new();

    let mut row_count = 0;
    for (y, row) in input.lines().enumerate() {
        row_count += 1;
        for (x, ch) in row.chars().enumerate() {
            if ch.is_alphanumeric() {
                antennas.push((x, y, ch));
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for x in 0..input.lines().next().unwrap().len() {
        for y in 0..row_count {
            for i in 0..antennas.len() {
                for j in 0..antennas.len() {
                    if i == j {
                        continue;
                    }
                    let (x1, y1, freq1) = antennas[i];
                    let (x2, y2, freq2) = antennas[j];

                    if freq1 != freq2 {
                        continue;
                    }

                    let points: Vec<(isize, isize)> = vec![
                        (x as isize, y as isize),
                        (x1 as isize, y1 as isize),
                        (x2 as isize, y2 as isize),
                    ];

                    // Ensure the second antenna is twice as far from the antinode
                    if points_are_linear(&points[..]) {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
    }

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
