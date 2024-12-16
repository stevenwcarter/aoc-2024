#![allow(unused_assignments)]
use num::Zero;

use hashbrown::{HashMap, HashSet};

use std::hash::Hash;
use std::{cmp::Reverse, collections::BinaryHeap};

advent_of_code::solution!(16);

pub fn dijkstra<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut success: FS,
) -> Option<(Vec<Vec<N>>, C)>
where
    N: Eq + Hash + Clone + Ord,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    dijkstra_internal(start, &mut successors, &mut success)
}

pub(crate) fn dijkstra_internal<N, C, FN, IN, FS>(
    start: &N,
    successors: &mut FN,
    success: &mut FS,
) -> Option<(Vec<Vec<N>>, C)>
where
    N: Eq + Hash + Clone + Ord,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let zero = C::zero();
    let mut dist = HashMap::new();
    let mut parents: HashMap<N, Vec<N>> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start.clone(), zero);
    heap.push(Reverse((zero, start.clone())));

    while let Some(Reverse((current_cost, current_node))) = heap.pop() {
        if current_cost > *dist.get(&current_node).unwrap_or(&C::zero()) {
            continue;
        }

        // goal reached
        if success(&current_node) {
            let paths = reconstruct_all_paths(&current_node, &parents);
            return Some((paths, current_cost));
        }

        // iter over successors
        for (neighbor, cost) in successors(&current_node) {
            let new_cost = current_cost + cost;

            if dist.get(&neighbor).map_or(true, |&c| new_cost < c) {
                // update if a shorter path is found
                dist.insert(neighbor.clone(), new_cost);
                parents.insert(neighbor.clone(), vec![current_node.clone()]);
                heap.push(Reverse((new_cost, neighbor)));
            } else if dist.get(&neighbor).map_or(false, |&c| new_cost == c) {
                // add if path cost is the same
                parents
                    .entry(neighbor.clone())
                    .or_default()
                    .push(current_node.clone());
            }
        }
    }

    None
}

fn reconstruct_all_paths<N>(goal: &N, parents: &HashMap<N, Vec<N>>) -> Vec<Vec<N>>
where
    N: Eq + Hash + Clone,
{
    fn backtrack<N>(
        node: &N,
        parents: &HashMap<N, Vec<N>>,
        current_path: &mut Vec<N>,
        all_paths: &mut Vec<Vec<N>>,
    ) where
        N: Eq + Hash + Clone,
    {
        current_path.push(node.clone());
        if let Some(parents_list) = parents.get(node) {
            // recurse on all parents of the current node
            for parent in parents_list {
                backtrack(parent, parents, current_path, all_paths);
            }
        } else {
            // if there are no parents, we've reached the start node
            all_paths.push(current_path.clone());
        }
        current_path.pop(); // Backtrack
    }

    let mut all_paths = Vec::new();
    let mut current_path = Vec::new();

    backtrack(goal, parents, &mut current_path, &mut all_paths);

    for path in &mut all_paths {
        path.reverse();
    }

    all_paths
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: u32,
    y: u32,
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as u32,
            y: value.1 as u32,
        }
    }
}
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

use Direction::*;

pub struct Maze {
    pub walls: HashSet<Point>,
    pub width: usize,
    pub height: usize,
    pub goal: Point,
    pub position: Point,
    pub facing: Direction,
}

impl Maze {
    pub fn parse_input(input: &str) -> Self {
        let input = &input[0..input.len() - 1];
        let width = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>()
            .len();
        let height = input.lines().collect::<Vec<_>>().len();
        let mut start: Option<Point> = None;
        let mut goal: Option<Point> = None;
        let walls: HashSet<Point> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, ch)| match ch {
                        'S' => {
                            start = Some((x, y).into());
                            None
                        }
                        'E' => {
                            goal = Some((x, y).into());
                            None
                        }
                        '#' => Some((x, y).into()),
                        _ => None,
                    })
                    .collect::<HashSet<Point>>()
            })
            .collect();

        Self {
            walls,
            width,
            height,
            position: start.unwrap(),
            goal: goal.unwrap(),
            facing: Direction::East,
        }
    }

    pub fn successors(
        &self,
        position: &Point,
        facing: &Direction,
    ) -> Vec<((Point, Direction), u32)> {
        let mut right_dir: Option<Direction> = None;
        let mut left_dir: Option<Direction> = None;
        let advancement_step: Option<Point> = match facing {
            North => {
                right_dir = Some(East);
                left_dir = Some(West);
                if position.y == 0 {
                    None
                } else {
                    Some((position.x, position.y - 1).into())
                }
            }
            South => {
                right_dir = Some(West);
                left_dir = Some(East);
                if position.y == (self.height - 1) as u32 {
                    None
                } else {
                    Some((position.x, position.y + 1).into())
                }
            }
            East => {
                right_dir = Some(South);
                left_dir = Some(North);
                if position.x == (self.width - 1) as u32 {
                    None
                } else {
                    Some((position.x + 1, position.y).into())
                }
            }
            West => {
                right_dir = Some(North);
                left_dir = Some(South);
                if position.x == 0 {
                    None
                } else {
                    Some((position.x - 1, position.y).into())
                }
            }
        };
        let mut options: Vec<((Point, Direction), u32)> = vec![
            ((*position, left_dir.unwrap()), 1000),
            ((*position, right_dir.unwrap()), 1000),
        ];
        if let Some(advancement_step) = advancement_step {
            if !self.walls.contains(&advancement_step) {
                options.push(((advancement_step, *facing), 1));
            }
        }

        options
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::parse_input(input);
    let start: (Point, Direction) = (maze.position, maze.facing);

    let result = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |(position, facing)| maze.successors(position, facing),
        |&(position, _)| position == maze.goal,
    );

    // println!("{:#?}", result);
    Some(result.unwrap().1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = Maze::parse_input(input);
    let start: (Point, Direction) = (maze.position, maze.facing);

    let results = dijkstra(
        &start,
        |(position, facing)| maze.successors(position, facing),
        |&(position, _)| position == maze.goal,
    )
    .unwrap();

    let mut visited: HashSet<Point> = HashSet::new();

    results.0.iter().for_each(|h| {
        h.iter().for_each(|(p, _)| {
            visited.insert(*p);
        });
    });

    let path_count = visited.len();
    assert!(path_count < 584);
    assert!(path_count > 485);
    Some(path_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
