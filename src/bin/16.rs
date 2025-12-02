#![allow(unused_assignments)]
use aoc_mine::Coord;
use num::Zero;

use hashbrown::{HashMap, HashSet};

use std::hash::Hash;
use std::time::Instant;
use std::{cmp::Reverse, collections::BinaryHeap};

advent_of_code::solution!(16);

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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

            if dist.get(&neighbor).is_none_or(|&c| new_cost < c) {
                // update if a shorter path is found
                dist.insert(neighbor.clone(), new_cost);
                parents.insert(neighbor.clone(), vec![current_node.clone()]);
                heap.push(Reverse((new_cost, neighbor)));
            } else if dist.get(&neighbor).is_some_and(|&c| new_cost == c) {
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

use advent_of_code::CardinalDirection::{self, *};

pub struct Maze {
    pub walls: HashSet<Coord<usize>>,
    pub width: usize,
    pub height: usize,
    pub goal: Coord<usize>,
    pub position: Coord<usize>,
    pub facing: CardinalDirection,
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
        let mut start: Option<Coord<usize>> = None;
        let mut goal: Option<Coord<usize>> = None;
        let walls: HashSet<Coord<usize>> = input
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
                    .collect::<HashSet<Coord<usize>>>()
            })
            .collect();

        Self {
            walls,
            width,
            height,
            position: start.unwrap(),
            goal: goal.unwrap(),
            facing: CardinalDirection::East,
        }
    }

    pub fn successors(
        &self,
        position: &Coord<usize>,
        facing: &CardinalDirection,
    ) -> Vec<((Coord<usize>, CardinalDirection), u32)> {
        let mut right_dir: Option<CardinalDirection> = None;
        let mut left_dir: Option<CardinalDirection> = None;
        let advancement_step: Option<Coord<usize>> = match facing {
            North => {
                right_dir = Some(East);
                left_dir = Some(West);
                position.up(Some(0))
            }
            South => {
                right_dir = Some(West);
                left_dir = Some(East);
                position.down(Some(self.height - 1))
            }
            East => {
                right_dir = Some(South);
                left_dir = Some(North);
                position.right(Some(self.width - 1))
            }
            West => {
                right_dir = Some(North);
                left_dir = Some(South);
                position.left(Some(0))
            }
        };
        let mut options: Vec<((Coord<usize>, CardinalDirection), u32)> = vec![
            ((*position, left_dir.unwrap()), 1000),
            ((*position, right_dir.unwrap()), 1000),
        ];
        if let Some(advancement_step) = advancement_step
            && !self.walls.contains(&advancement_step)
        {
            options.push(((advancement_step, *facing), 1));
        }

        options
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::parse_input(input);
    let start: (Coord<usize>, CardinalDirection) = (maze.position, maze.facing);

    Some(
        pathfinding::directed::dijkstra::dijkstra(
            &start,
            |(position, facing)| maze.successors(position, facing),
            |&(position, _)| position == maze.goal,
        )
        .unwrap()
        .1,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = Maze::parse_input(input);
    let start: (Coord<usize>, CardinalDirection) = (maze.position, maze.facing);

    let results = dijkstra(
        &start,
        |(position, facing)| maze.successors(position, facing),
        |&(position, _)| position == maze.goal,
    )
    .unwrap();

    let mut visited: HashSet<Coord<usize>> = HashSet::new();

    results.0.iter().for_each(|h| {
        h.iter().for_each(|(p, _)| {
            visited.insert(*p);
        });
    });

    let path_count = visited.len();
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
