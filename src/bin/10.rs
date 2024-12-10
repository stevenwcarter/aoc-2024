use hashbrown::HashSet;

advent_of_code::solution!(10);

#[derive(Default, Debug)]
pub struct TopoMap {
    pub grid: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

impl TopoMap {
    pub fn new(input: &str) -> Self {
        let grid: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap_or(11) as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            height,
            width,
        }
    }

    pub fn possible_trailheads(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, d)| **d == 0)
                    .map(|(x, _)| (x, y))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect()
    }

    pub fn find_paths_from_coord(
        &self,
        coord: (usize, usize),
        next_val: u8,
    ) -> Vec<(usize, usize)> {
        let (x, y) = coord;

        let mut coords: Vec<(usize, usize)> = Vec::new();
        if x > 0 && self.grid[y][x - 1] == next_val {
            coords.push((x - 1, y));
        }
        if y > 0 && self.grid[y - 1][x] == next_val {
            coords.push((x, y - 1));
        }
        if y < self.height - 1 && self.grid[y + 1][x] == next_val {
            coords.push((x, y + 1));
        }
        if x < self.width - 1 && self.grid[y][x + 1] == next_val {
            coords.push((x + 1, y));
        }

        coords
    }

    pub fn follow_trail(&self, coord: (usize, usize), next_val: u8) -> u32 {
        self.find_paths_from_coord(coord, next_val)
            .iter()
            .map(|(x, y)| {
                if next_val == 9 {
                    1
                } else {
                    self.follow_trail((*x, *y), next_val + 1)
                }
            })
            .sum()
    }

    pub fn count_trailheads(&self) -> Option<u32> {
        let possible_trailheads = self.possible_trailheads();
        Some(
            possible_trailheads
                .iter()
                .map(|coord| self.follow_trail(*coord, 1))
                .sum(),
        )
    }
    pub fn find_paths_from_coord_part1(
        &self,
        coord: (usize, usize),
        next_val: u8,
    ) -> Vec<(usize, usize)> {
        let (x, y) = coord;

        let mut coords: Vec<(usize, usize)> = Vec::new();
        if x > 0 && self.grid[y][x - 1] == next_val {
            coords.push((x - 1, y));
        }
        if y > 0 && self.grid[y - 1][x] == next_val {
            coords.push((x, y - 1));
        }
        if y < self.height - 1 && self.grid[y + 1][x] == next_val {
            coords.push((x, y + 1));
        }
        if x < self.width - 1 && self.grid[y][x + 1] == next_val {
            coords.push((x + 1, y));
        }

        coords
    }

    pub fn follow_trail_part1(
        &self,
        coord: (usize, usize),
        next_val: u8,
        result: &mut HashSet<(usize, usize)>,
    ) {
        self.find_paths_from_coord_part1(coord, next_val)
            .iter()
            .for_each(|(x, y)| {
                if next_val == 9 {
                    result.insert((*x, *y));
                } else {
                    self.follow_trail_part1((*x, *y), next_val + 1, result);
                }
            });
    }

    pub fn count_trailheads_part1(&self) -> Option<usize> {
        let possible_trailheads = self.possible_trailheads();
        Some(
            possible_trailheads
                .iter()
                .map(|coord| {
                    let mut result: HashSet<(usize, usize)> = HashSet::new();
                    self.follow_trail_part1(*coord, 1, &mut result);

                    result.len()
                })
                .sum(),
        )
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let topo_map = TopoMap::new(input);

    topo_map.count_trailheads_part1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let topo_map = TopoMap::new(input);

    topo_map.count_trailheads()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(36));
    }
    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(81));
    }
}
