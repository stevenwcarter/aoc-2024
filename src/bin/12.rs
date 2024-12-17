#![allow(unused_imports)]
use advent_of_code::Point;
use hashbrown::{HashMap, HashSet};

use rayon::prelude::*;
advent_of_code::solution!(12);

fn count_corners(grid: &HashSet<Point>) -> usize {
    let mut edge_count = 0;
    for &pt in grid {
        let up = pt.up(None);
        let down = pt.down(None);
        let left = pt.left(None);
        let right = pt.right(None);
        let up_right = pt.up_right(None, None);
        let up_left = pt.up_left(None, None);
        let down_right = pt.down_right(None, None);
        let down_left = pt.down_left(None, None);
        let up = up.is_some() && grid.contains(&up.unwrap());
        let down = down.is_some() && grid.contains(&down.unwrap());
        let left = left.is_some() && grid.contains(&left.unwrap());
        let right = right.is_some() && grid.contains(&right.unwrap());
        let up_left = up_left.is_some() && grid.contains(&up_left.unwrap());
        let up_right = up_right.is_some() && grid.contains(&up_right.unwrap());
        let down_left = down_left.is_some() && grid.contains(&down_left.unwrap());
        let down_right = down_right.is_some() && grid.contains(&down_right.unwrap());

        if !up && !right || up && right && !up_right {
            edge_count += 1;
        }
        if !up && !left || up && left && !up_left {
            edge_count += 1;
        }
        if !down && !right || down && right && !down_right {
            edge_count += 1;
        }
        if !down && !left || down && left && !down_left {
            edge_count += 1;
        }
    }

    edge_count
}

pub struct Garden {
    pub grid: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Garden {
    pub fn parse(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            width,
            height,
        }
    }

    pub fn find_neighbor_count(&self, point: Point, ch: &char) -> usize {
        let mut neighbor_count = 0;
        let (x, y) = (point.x as usize, point.y as usize);
        if x == 0 || &self.grid[y][x - 1] != ch {
            neighbor_count += 1;
        }
        if y == 0 || &self.grid[y - 1][x] != ch {
            neighbor_count += 1;
        }
        if x >= self.width - 1 || &self.grid[y][x + 1] != ch {
            neighbor_count += 1;
        }
        if y >= self.height - 1 || &self.grid[y + 1][x] != ch {
            neighbor_count += 1;
        }

        neighbor_count
    }

    pub fn find_neighbors(
        &self,
        x: usize,
        y: usize,
        ch: &char,
        visited: &mut HashMap<Point, bool>,
    ) -> HashSet<Point> {
        let mut neighbors = HashSet::new();
        let mut stack: Vec<Point> = vec![(x, y).into()];
        visited.insert((x, y).into(), true);

        while let Some(point) = stack.pop() {
            neighbors.insert(point);

            let directions = [
                point.up(None),
                point.right(Some(self.width as u32)),
                point.down(Some(self.height as u32)),
                point.left(None),
            ];

            directions.iter().filter_map(|pt| *pt).for_each(|point| {
                let (nx, ny) = (point.x as usize, point.y as usize);
                if self.grid[ny][nx] == *ch && !*visited.get(&point).unwrap_or(&false) {
                    stack.push(point);
                    visited.insert(point, true);
                }
            });
        }

        neighbors
    }

    pub fn find_areas(&self, is_part_2: bool) -> Vec<(usize, usize, char)> {
        let mut area_perimeters: Vec<(usize, usize, char)> = Vec::new();
        let mut visited: HashMap<Point, bool> = HashMap::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if !visited.get(&Point::from((x, y))).unwrap_or(&false) {
                    let neighbors = self.find_neighbors(x, y, ch, &mut visited);

                    // find area
                    let area = neighbors.len();

                    // find perimeter
                    let perimeter = if is_part_2 {
                        count_corners(&neighbors)
                    } else {
                        neighbors
                            .iter()
                            .map(|pt| self.find_neighbor_count(*pt, ch))
                            .sum::<usize>()
                    };

                    area_perimeters.push((area, perimeter, *ch));
                }
            }
        }

        area_perimeters
    }

    pub fn fence_pricing(&self, is_part_2: bool) -> Option<usize> {
        let areas = self.find_areas(is_part_2);

        Some(areas.iter().map(|(a, p, _)| a * p).sum())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let garden = Garden::parse(input);

    garden.fence_pricing(false)
}

pub fn part_two(input: &str) -> Option<usize> {
    let garden = Garden::parse(input);

    garden.fence_pricing(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_corners() {
        let mut points: HashSet<Point> = HashSet::new();
        points.insert(Point::from((0u32, 0)));
        points.insert(Point::from((0u32, 1)));
        points.insert(Point::from((0u32, 2)));
        points.insert(Point::from((2u32, 0)));
        points.insert(Point::from((2u32, 1)));
        points.insert(Point::from((2u32, 2)));
        points.insert(Point::from((1u32, 0)));
        points.insert(Point::from((1u32, 2)));

        assert_eq!(count_corners(&points), 8);
    }
    #[test]
    fn test_count_corners_square() {
        let points: HashSet<Point> = vec![Point::from((0u32, 0))].into_iter().collect();
        assert_eq!(count_corners(&points), 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(368));
    }
}
