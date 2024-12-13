use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(12);

fn count_corners(grid: &HashSet<(usize, usize)>) -> usize {
    let mut edge_count = 0;
    for &(x, y) in grid {
        let up = y > 0 && grid.contains(&(x, y - 1));
        let down = grid.contains(&(x, y + 1));
        let left = x > 0 && grid.contains(&(x - 1, y));
        let right = grid.contains(&(x + 1, y));
        let up_left = y > 0 && x > 0 && grid.contains(&(x - 1, y - 1));
        let up_right = y > 0 && grid.contains(&(x + 1, y - 1));
        let down_left = x > 0 && grid.contains(&(x - 1, y + 1));
        let down_right = grid.contains(&(x + 1, y + 1));

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

    pub fn find_neighbor_count(&self, x: usize, y: usize, ch: &char) -> usize {
        let mut neighbor_count = 0;
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
        visited: &mut HashMap<(usize, usize), bool>,
    ) -> Option<HashSet<(usize, usize)>> {
        if *visited.get(&(x, y)).unwrap_or(&false) {
            None
        } else {
            let mut neighbors: HashSet<(usize, usize)> = vec![(x, y)].into_iter().collect();
            // neighbors.insert((x, y));
            *visited.entry((x, y)).or_insert(true) = true;
            if x > 0 && self.grid[y][x - 1] == *ch {
                if let Some(extension) = self.find_neighbors(x - 1, y, ch, visited) {
                    neighbors.extend(extension);
                }
            }
            if y > 0 && self.grid[y - 1][x] == *ch {
                if let Some(extension) = self.find_neighbors(x, y - 1, ch, visited) {
                    neighbors.extend(extension);
                }
            }
            if x < self.width - 1 && self.grid[y][x + 1] == *ch {
                if let Some(extension) = self.find_neighbors(x + 1, y, ch, visited) {
                    neighbors.extend(extension);
                }
            }
            if y < self.height - 1 && self.grid[y + 1][x] == *ch {
                if let Some(extension) = self.find_neighbors(x, y + 1, ch, visited) {
                    neighbors.extend(extension);
                }
            }

            Some(neighbors)
        }
    }

    pub fn find_areas(&self, is_part_2: bool) -> Vec<(usize, usize, char)> {
        let mut area_perimeters: Vec<(usize, usize, char)> = Vec::new();
        let mut visited: HashMap<(usize, usize), bool> = HashMap::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if !visited.get(&(x, y)).unwrap_or(&false) {
                    let neighbors = self.find_neighbors(x, y, ch, &mut visited).unwrap();

                    // find area
                    let area = neighbors.len();

                    // find perimeter
                    let perimeter = if is_part_2 {
                        count_corners(&neighbors)
                    } else {
                        neighbors
                            .iter()
                            .map(|(x, y)| self.find_neighbor_count(*x, *y, ch))
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
        let mut points: HashSet<(usize, usize)> = HashSet::new();
        points.insert((0, 0));
        points.insert((0, 1));
        points.insert((0, 2));
        points.insert((2, 0));
        points.insert((2, 1));
        points.insert((2, 2));
        points.insert((1, 0));
        points.insert((1, 2));

        assert_eq!(count_corners(&points), 8);
    }
    #[test]
    fn test_count_corners_square() {
        let points: HashSet<(usize, usize)> = vec![(0, 0)].into_iter().collect();
        assert_eq!(count_corners(&points), 4);
    }
    #[test]
    fn test_count_corners_larger_square() {
        let points: HashSet<(usize, usize)> =
            vec![(0, 0), (0, 1), (1, 0), (1, 1)].into_iter().collect();
        assert_eq!(count_corners(&points), 4);
    }
    #[test]
    fn test_count_corners_e() {
        let points: HashSet<(usize, usize)> =
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (0, 2), (1, 2), (2, 2)]
                .into_iter()
                .collect();
        assert_eq!(count_corners(&points), 8);
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
