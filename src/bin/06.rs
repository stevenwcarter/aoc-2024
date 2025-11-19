#![allow(unused_imports)]
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;

use aoc_mine::{Coord, Grid, HashGrid, LinearGrid};

advent_of_code::solution!(6);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone)]
pub struct State<T: Grid<u8, SquareType>> {
    pub grid: T,
    pub guard_pos: Coord<u8>,
    pub guard_facing: Direction,
    pub visited: HashMap<Coord<u8>, bool>,
    pub visited_obstacles: HashSet<(Coord<u8>, Direction)>,
    pub steps: usize,
    pub width: usize,
    pub height: usize,
}

impl State<LinearGrid<u8, SquareType>> {
    pub fn new_from_input(input: &str) -> Self {
        let mut guard_pos: Option<Coord<u8>> = None;
        let height = input.lines().collect::<Vec<_>>().len();
        let width = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>()
            .len();
        // let mut grid: HashGrid<u8, SquareType> = HashGrid::new().set_min_x(0).set_min_y(0);
        // grid = grid.set_max_x((width - 1) as u8);
        // grid = grid.set_max_y((height - 1) as u8);
        let mut grid: LinearGrid<u8, SquareType> =
            LinearGrid::new(width, height, SquareType::Clear);
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let square_type = match c {
                    '.' => SquareType::Clear,
                    '#' => SquareType::Obstacle,
                    '^' => {
                        guard_pos = Some((x as u8, y as u8).into());
                        SquareType::Clear
                    }
                    _ => unreachable!("unknown symbol {:?}", c),
                };
                let _ = grid.insert((x as u8, y as u8).into(), square_type);
            }
        }

        Self {
            grid,
            guard_pos: guard_pos.expect("no guard position found in input"),
            guard_facing: Direction::Up,
            visited: HashMap::new(),
            visited_obstacles: HashSet::new(),
            steps: 0,
            width,
            height,
        }
    }
}
impl<T: Grid<u8, SquareType>> State<T> {
    fn turn(&mut self) {
        self.guard_facing = match self.guard_facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_block(&self) -> Option<Coord<u8>> {
        let current_pos = self.guard_pos;
        match self.guard_facing {
            Direction::Up => current_pos.up(),
            Direction::Right => current_pos.right(),
            Direction::Down => current_pos.down(),
            Direction::Left => current_pos.left(),
        }
    }
    fn next_block2(&mut self) -> Option<Coord<u8>> {
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
                        let y = y?;
                        if y == 0 {
                            return None;
                        }
                        let result = self
                            .grid
                            .matches(&(current_pos.0, y).into(), SquareType::Obstacle)
                            .unwrap_or(false);
                        if result {
                            return Some(Coord(current_pos.0, y));
                        }
                        let y2 = y2?;
                        let result2 = self
                            .grid
                            .matches(&(current_pos.0, y2).into(), SquareType::Obstacle)
                            .ok()?;

                        if result2 {
                            updated_guard_pos = Some(Coord(current_pos.0, y));
                            Some(Coord(current_pos.0, y2))
                        } else {
                            None
                        }
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
                        let x = x? as u8;
                        if x == 0 {
                            return None;
                        }
                        let result = self
                            .grid
                            .matches(&(x, current_pos.1).into(), SquareType::Obstacle)
                            .unwrap_or(false);
                        if result {
                            return Some(Coord(x, current_pos.1));
                        }
                        let x2 = x2? as u8;
                        let result2 = self
                            .grid
                            .matches(&(x2, current_pos.1).into(), SquareType::Obstacle)
                            .ok()?;

                        if result2 {
                            updated_guard_pos = Some(Coord(x, current_pos.1));
                            Some(Coord(x2, current_pos.1))
                        } else {
                            None
                        }
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
                        let y = y? as u8;
                        if y == 0 {
                            return None;
                        }
                        let result = self
                            .grid
                            .matches(&(current_pos.0, y).into(), SquareType::Obstacle)
                            .unwrap_or(false);
                        if result {
                            return Some(Coord(current_pos.0, y));
                        }
                        let y2 = y2? as u8;
                        let result2 = self
                            .grid
                            .matches(&(current_pos.0, y2).into(), SquareType::Obstacle)
                            .ok()?;

                        if result2 {
                            updated_guard_pos = Some(Coord(current_pos.0, y));
                            Some(Coord(current_pos.0, y2))
                        } else {
                            None
                        }
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
                        let x = x?;
                        if x == 0 {
                            return None;
                        }
                        let result = self
                            .grid
                            .matches(&(x, current_pos.1).into(), SquareType::Obstacle)
                            .unwrap_or(false);
                        if result {
                            return Some(Coord(x, current_pos.1));
                        }
                        let x2 = x2?;
                        let result2 = self
                            .grid
                            .matches(&(x2, current_pos.1).into(), SquareType::Obstacle)
                            .ok()?;

                        if result2 {
                            updated_guard_pos = Some(Coord(x, current_pos.1));
                            Some(Coord(x2, current_pos.1))
                        } else {
                            None
                        }
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
        let next_pos = self.next_block()?;

        self.grid.get(&next_pos)
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
        if next_block.is_none() {
            return Some(false);
        }
        self.visited.entry(self.guard_pos).or_insert(true);
        let next_block_type = self.next_block_type();

        if next_block_type.is_none() {
            return Some(false);
        }

        match next_block_type? {
            SquareType::Clear => {
                self.guard_pos = next_block.unwrap();
            }
            SquareType::Obstacle => {
                match self
                    .visited_obstacles
                    .insert((self.guard_pos, self.guard_facing))
                {
                    false => return Some(true),
                    true => self.turn(),
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
    let clear_areas: Vec<Coord<u8>> = check_state.visited.keys().copied().collect();

    let valid_loops = clear_areas
        .par_iter()
        .filter(|coord| {
            let mut state = state.clone();

            let _ = state.grid.insert(**coord, SquareType::Obstacle);
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
