use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(6);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquareType {
    Obstacle,
    Clear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const MAX_ITERS: usize = 6000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(u8, u8);

impl Coord {
    pub fn x(&self) -> u8 {
        self.0
    }
    pub fn y(&self) -> u8 {
        self.1
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub grid: HashMap<Coord, SquareType>,
    pub guard_pos: Coord,
    pub guard_facing: Direction,
    pub visited: HashMap<Coord, bool>,
    pub visited_obstacles: HashSet<(Coord, Direction)>,
    pub steps: usize,
    pub width: usize,
    pub height: usize,
}

impl State {
    pub fn new_from_input(input: &str) -> Self {
        let mut guard_pos: Option<Coord> = None;
        let mut grid: HashMap<Coord, SquareType> = HashMap::new();
        let height = input.lines().collect::<Vec<_>>().len();
        let width = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>()
            .len();
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let square_type = match c {
                    '.' => SquareType::Clear,
                    '#' => SquareType::Obstacle,
                    '^' => {
                        guard_pos = Some(Coord(x as u8, y as u8));
                        SquareType::Clear
                    }
                    _ => unreachable!("unknown symbol {:?}", c),
                };
                grid.entry(Coord(x as u8, y as u8)).or_insert(square_type);
            }
        }

        Self {
            grid,
            guard_pos: guard_pos.unwrap(),
            guard_facing: Direction::Up,
            visited: HashMap::new(),
            visited_obstacles: HashSet::new(),
            steps: 0,
            width,
            height,
        }
    }

    fn turn(&mut self) {
        self.guard_facing = match self.guard_facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_block(&self) -> Option<Coord> {
        let current_pos = self.guard_pos;
        match self.guard_facing {
            Direction::Up => {
                if current_pos.1 == 0 {
                    None
                } else {
                    Some(Coord(current_pos.0, current_pos.1 - 1))
                }
            }
            Direction::Right => Some(Coord(current_pos.0 + 1, current_pos.1)),
            Direction::Down => Some(Coord(current_pos.0, current_pos.1 + 1)),
            Direction::Left => {
                if current_pos.0 == 0 {
                    None
                } else {
                    Some(Coord(current_pos.0 - 1, current_pos.1))
                }
            }
        }
    }
    fn next_block2(&mut self) -> Option<Coord> {
        let current_pos = self.guard_pos;
        match self.guard_facing {
            Direction::Up => {
                let mut updated_guard_pos = None;
                let next_obstacle_y = (0..current_pos.1)
                    .rev()
                    .map(Some)
                    .chain([None])
                    .tuple_windows()
                    .filter_map(|(y, y2)| {
                        y?;
                        let y = y.unwrap();
                        if y == 0 {
                            return None;
                        }
                        let result =
                            self.grid.get(&Coord(current_pos.0, y)) == Some(&SquareType::Obstacle);
                        if result {
                            return Some(Coord(current_pos.0, y));
                        }
                        y2?;
                        let y2 = y2.unwrap();
                        let result2 =
                            self.grid.get(&Coord(current_pos.0, y2)) == Some(&SquareType::Obstacle);

                        if result2 {
                            updated_guard_pos = Some(Coord(current_pos.0, y));
                        } else {
                            return None;
                        }

                        Some(Coord(current_pos.0, y2))
                    })
                    .next();

                if let Some(updated_position) = updated_guard_pos {
                    self.guard_pos = updated_position;
                }
                next_obstacle_y
            }
            Direction::Right => {
                let mut updated_guard_pos = None;
                let next_obstacle_x = (current_pos.0 as usize + 1..self.width)
                    .map(Some)
                    .chain([None])
                    .tuple_windows()
                    .filter_map(|(x, x2)| {
                        x?;
                        let x = x.unwrap() as u8;
                        if x == 0 {
                            return None;
                        }
                        let result =
                            self.grid.get(&Coord(x, current_pos.1)) == Some(&SquareType::Obstacle);
                        if result {
                            return Some(Coord(x, current_pos.1));
                        }
                        x2?;
                        let x2 = x2.unwrap() as u8;
                        let result2 =
                            self.grid.get(&Coord(x2, current_pos.1)) == Some(&SquareType::Obstacle);

                        if result2 {
                            updated_guard_pos = Some(Coord(x, current_pos.1));
                        } else {
                            return None;
                        }

                        Some(Coord(x2, current_pos.1))
                    })
                    .next();

                if let Some(updated_position) = updated_guard_pos {
                    self.guard_pos = updated_position;
                }
                next_obstacle_x
            }
            Direction::Down => {
                let mut updated_guard_pos = None;
                let next_obstacle_y = (current_pos.1 as usize + 1..self.height)
                    .map(Some)
                    .chain([None])
                    .tuple_windows()
                    .filter_map(|(y, y2)| {
                        y?;
                        let y = y.unwrap() as u8;
                        if y == 0 {
                            return None;
                        }
                        let result =
                            self.grid.get(&Coord(current_pos.0, y)) == Some(&SquareType::Obstacle);
                        if result {
                            return Some(Coord(current_pos.0, y));
                        }
                        y2?;
                        let y2 = y2.unwrap() as u8;
                        let result2 =
                            self.grid.get(&Coord(current_pos.0, y2)) == Some(&SquareType::Obstacle);

                        if result2 {
                            updated_guard_pos = Some(Coord(current_pos.0, y));
                        } else {
                            return None;
                        }

                        Some(Coord(current_pos.0, y2))
                    })
                    .next();

                if let Some(updated_position) = updated_guard_pos {
                    self.guard_pos = updated_position;
                }
                next_obstacle_y
            }
            Direction::Left => {
                let mut updated_guard_pos = None;
                let next_obstacle_x = (0..current_pos.0)
                    .rev()
                    .map(Some)
                    .chain([None])
                    .tuple_windows()
                    .filter_map(|(x, x2)| {
                        x?;
                        let x = x.unwrap();
                        if x == 0 {
                            return None;
                        }
                        let result =
                            self.grid.get(&Coord(x, current_pos.1)) == Some(&SquareType::Obstacle);
                        if result {
                            return Some(Coord(x, current_pos.1));
                        }
                        x2?;
                        let x2 = x2.unwrap();
                        let result2 =
                            self.grid.get(&Coord(x2, current_pos.1)) == Some(&SquareType::Obstacle);

                        if result2 {
                            updated_guard_pos = Some(Coord(x, current_pos.1));
                        } else {
                            return None;
                        }

                        Some(Coord(x2, current_pos.1))
                    })
                    .next();

                if let Some(updated_position) = updated_guard_pos {
                    self.guard_pos = updated_position;
                }
                next_obstacle_x
            }
        }
    }

    fn next_block_type(&self) -> Option<&SquareType> {
        let next_pos = self.next_block();

        match next_pos {
            Some(coord) => self.grid.get(&coord),
            None => None,
        }
    }

    pub fn step(&mut self) -> bool {
        self.steps += 1;
        if self.steps > MAX_ITERS {
            return false;
        }
        self.visited.entry(self.guard_pos).or_insert(true);
        let next_block = self.next_block();
        let next_block_type = self.next_block_type();
        if next_block.is_none() || next_block_type.is_none() {
            return false;
        }
        let next_block_type = next_block_type.unwrap();
        let next_block = next_block.unwrap();
        match next_block_type {
            SquareType::Clear => {
                self.guard_pos = next_block;
            }
            SquareType::Obstacle => {
                self.turn();
            }
        }

        true
    }
    pub fn step2(&mut self) -> Option<bool> {
        self.steps += 1;
        if self.steps > MAX_ITERS {
            return Some(false);
        }
        let next_block = self.next_block2();
        self.visited.entry(self.guard_pos).or_insert(true);
        let next_block_type = self.next_block_type();
        // println!("Next block type: {:?}", next_block_type);
        if next_block.is_none() || next_block_type.is_none() {
            return Some(false);
        }
        let next_block = next_block?;
        match next_block_type? {
            SquareType::Clear => {
                self.guard_pos = next_block;
            }
            SquareType::Obstacle => {
                if !self
                    .visited_obstacles
                    .insert((self.guard_pos, self.guard_facing))
                {
                    return Some(true);
                } else {
                    self.turn();
                }
            }
        }

        None
    }

    pub fn count_visited(&self) -> usize {
        self.visited.iter().filter(|(_, val)| **val).count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut state = State::new_from_input(input);
    while state.step() {
        // loop until it leaves the area
    }
    Some(state.count_visited())
}

pub fn part_two(input: &str) -> Option<usize> {
    let state = State::new_from_input(input);
    let mut check_state = state.clone();
    while check_state.step() {
        // run once to find visited areas, since those are the
        // only places we could place an obstruction that would
        // change the path
    }
    let clear_areas: Vec<Coord> = check_state.visited.keys().copied().collect();

    let valid_loops = clear_areas
        .par_iter()
        .filter(|coord| {
            let mut state = state.clone();
            *state.grid.entry(**coord).or_insert(SquareType::Obstacle) = SquareType::Obstacle;
            loop {
                if let Some(is_loop) = state.step2() {
                    return is_loop;
                }
            }
        })
        .count();

    Some(valid_loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
