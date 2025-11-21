use aoc_mine::{Coord, Grid, LinearGrid};

advent_of_code::solution!(15);

use advent_of_code::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Open,
    Robot,
}

#[derive(Debug, Clone)]
pub struct Warehouse {
    pub grid: LinearGrid<usize, BlockType>,
    pub robot_position: Coord<usize>,
    pub width: usize,
    pub height: usize,
    pub directions: Vec<Direction>,
}

impl Warehouse {
    pub fn parse_input(input: &str, part_2: bool) -> Self {
        let (graph, directions) = input.split_once("\n\n").unwrap();
        let mut robot_position = None;
        let mut width = input.lines().next().unwrap().len();
        if part_2 {
            width *= 2;
        }
        let height = graph.lines().collect::<Vec<_>>().len();
        let mut grid: LinearGrid<usize, BlockType> =
            LinearGrid::new(width, height, BlockType::Open);
        if !part_2 {
            graph.lines().enumerate().for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, ch)| {
                    let mut block_type = match ch {
                        '#' => BlockType::Wall,
                        '@' => BlockType::Robot,
                        '.' => BlockType::Open,
                        'O' => BlockType::Box,
                        _ => unreachable!("Should not have received {ch}"),
                    };
                    if block_type == BlockType::Robot {
                        robot_position = Some((x, y).into());
                        block_type = BlockType::Open;
                    }

                    let _ = grid.insert(Coord::new(x, y), block_type);
                });
            });
        } else {
            graph.lines().enumerate().for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, ch)| {
                    let mut block_type_l = match ch {
                        '#' => BlockType::Wall,
                        '@' => BlockType::Robot,
                        '.' => BlockType::Open,
                        'O' => BlockType::Box,
                        _ => unreachable!("Should not have received {ch}"),
                    };
                    let block_type_r = match ch {
                        '#' => BlockType::Wall,
                        '@' => BlockType::Open,
                        '.' => BlockType::Open,
                        'O' => BlockType::BoxRight,
                        _ => unreachable!("Should not have received {ch}"),
                    };
                    if block_type_l == BlockType::Robot {
                        robot_position = Some((x * 2, y).into());
                        block_type_l = BlockType::Open;
                    }

                    let _ = grid.insert(Coord::new(x * 2, y), block_type_l);
                    let _ = grid.insert(Coord::new(x * 2 + 1, y), block_type_r);
                });
            });
        }

        let directions: Vec<Direction> = directions
            .replace('\n', "")
            .chars()
            .map(|ch| match ch {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => unreachable!("Invalid character in directions: {ch}"),
            })
            .collect();

        Self {
            grid,
            robot_position: robot_position.expect("did not find robot position"),
            width,
            height,
            directions,
        }
    }

    pub fn move_unchecked(&mut self, old_position: &Coord<usize>, new_position: &Coord<usize>) {
        let contents = *self.grid.get(old_position).unwrap();
        let _ = self.grid.insert(*old_position, BlockType::Open);
        let _ = self.grid.insert(*new_position, contents);
    }

    pub fn attempt_move(
        &mut self,
        position: &Coord<usize>,
        direction: Direction,
        is_robot: bool,
    ) -> bool {
        let next_position = match direction {
            Direction::Up => position.up(Some(0)),
            Direction::Down => position.down(Some(self.height - 1)),
            Direction::Left => position.left(Some(0)),
            Direction::Right => position.right(Some(self.width - 1)),
        };
        if next_position.is_none() {
            return false;
        }
        let next_position = next_position.unwrap();
        let can_move = match self.grid.get(&next_position).unwrap() {
            BlockType::Open => true,
            BlockType::Wall => false,
            BlockType::Box => self.attempt_move(&next_position, direction, false),
            _ => unreachable!("Should not be other types"),
        };

        if can_move {
            self.move_unchecked(position, &next_position);
            if is_robot {
                self.robot_position = next_position;
            }
        }

        can_move
    }
    pub fn attempt_move_part2(
        &mut self,
        position: &Coord<usize>,
        direction: Direction,
        is_robot: bool,
        skip_moving: bool,
    ) -> bool {
        let next_position = match direction {
            Direction::Up => position.up(Some(0)),
            Direction::Down => position.down(Some(self.height - 1)),
            Direction::Left => position.left(Some(0)),
            Direction::Right => position.right(Some(self.width - 1)),
        };
        if next_position.is_none() {
            return false;
        }
        let next_position = next_position.unwrap();

        let next_block_type = *self.grid.get(&next_position).unwrap();
        let can_move = match next_block_type {
            BlockType::Open => true,
            BlockType::Wall => false,
            BlockType::Box => {
                if direction == Direction::Up || direction == Direction::Down {
                    if !self.attempt_move_part2(&next_position, direction, false, true) {
                        false
                    } else {
                        let right_next_position = next_position.right(None).unwrap();
                        let right_side_can_move =
                            self.attempt_move_part2(&right_next_position, direction, false, true);
                        if right_side_can_move {
                            self.attempt_move_part2(&next_position, direction, false, skip_moving)
                                && self.attempt_move_part2(
                                    &right_next_position,
                                    direction,
                                    false,
                                    skip_moving,
                                )
                        } else {
                            false
                        }
                    }
                } else {
                    self.attempt_move_part2(&next_position, direction, false, skip_moving)
                }
            }
            BlockType::BoxRight => {
                if direction == Direction::Up || direction == Direction::Down {
                    if !self.attempt_move_part2(&next_position, direction, false, true) {
                        false
                    } else {
                        let left_next_position = next_position.left(None).unwrap();
                        let left_side_can_move =
                            self.attempt_move_part2(&left_next_position, direction, false, true);
                        if left_side_can_move {
                            // skip_moving = true;
                            self.attempt_move_part2(&next_position, direction, false, skip_moving)
                                && self.attempt_move_part2(
                                    &left_next_position,
                                    direction,
                                    false,
                                    skip_moving,
                                )
                        } else {
                            false
                        }
                    }
                } else {
                    self.attempt_move_part2(&next_position, direction, false, skip_moving)
                }
            }
            _ => unreachable!("Should not be other types"),
        };

        if can_move && !skip_moving {
            self.move_unchecked(position, &next_position);
            if is_robot {
                self.robot_position = next_position;
            }
        }

        can_move
    }

    pub fn follow_robot_directions(&mut self) {
        for direction in self.directions.clone() {
            let robot_position = self.robot_position;
            self.attempt_move(&robot_position, direction, true);
        }
    }
    pub fn follow_robot_directions_part2(&mut self) {
        for direction in self.directions.clone() {
            let robot_position = self.robot_position;
            self.attempt_move_part2(&robot_position, direction, true, false);
        }
    }

    pub fn coordinate_summation(&self) -> usize {
        self.grid
            .iter()
            .filter(|(_, p)| matches!(p, BlockType::Box))
            .map(|(p, _)| p.y() * 100 + p.x())
            .sum()
    }

    pub fn print(&self, part_2: bool) {
        println!("\n");
        (0..self.height).for_each(|y| {
            let line = (0..self.width)
                .map(|x| {
                    let p = Coord::new(x, y);
                    if p == self.robot_position {
                        return '@';
                    }
                    match self.grid.get(&p).unwrap_or(&BlockType::Open) {
                        BlockType::Box => {
                            if part_2 {
                                '['
                            } else {
                                'O'
                            }
                        }
                        BlockType::BoxRight => ']',
                        BlockType::Wall => '#',
                        BlockType::Open => '.',
                        _ => unreachable!("should not have this type"),
                    }
                })
                .collect::<String>();
            println!("{line}");
        });
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut warehouse = Warehouse::parse_input(input, false);

    warehouse.follow_robot_directions();

    // warehouse.print(true);

    Some(warehouse.coordinate_summation())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut warehouse = Warehouse::parse_input(input, true);

    warehouse.follow_robot_directions_part2();

    // warehouse.print(true);

    Some(warehouse.coordinate_summation())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_summation() {
        let mut grid: LinearGrid<usize, BlockType> = LinearGrid::new(10, 2, BlockType::Open);
        let _ = grid.insert(Coord::new(4, 1), BlockType::Box);
        let warehouse = Warehouse {
            grid,
            width: 10,
            height: 2,
            robot_position: (0, 0).into(),
            directions: vec![],
        };

        assert_eq!(warehouse.coordinate_summation(), 104);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }
    #[test]
    fn test_part_one_2() {
        let mut warehouse = Warehouse::parse_input(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
            false,
        );
        warehouse.follow_robot_directions();
        warehouse.print(false);
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1751));
    }
}
