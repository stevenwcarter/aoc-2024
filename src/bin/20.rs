advent_of_code::solution!(20);

use std::collections::VecDeque;

// TODO: Clean up this implementation

#[derive(Debug, Clone, Copy)]
pub struct Distances(Option<usize>, Option<usize>);

#[derive(Debug, Clone)]
pub enum GridTile {
    Open(Distances),
    Wall,
}

fn find_original_min_distances(tiles: &mut [GridTile], width: usize, start: usize, end: usize) {
    let mut queue = VecDeque::new();
    queue.push_front((start, 0));

    while let Some((position, distance)) = queue.pop_front() {
        if let GridTile::Open(distances) = &mut tiles[position] {
            if distances.0.is_none() {
                distances.0 = Some(distance);
                [
                    position + 1,
                    position - 1,
                    position + width,
                    position - width,
                ]
                .into_iter()
                .for_each(|new_position| queue.push_back((new_position, distance + 1)));
            }
        }

        if position == end {
            break;
        }
    }

    // reverse to find distances from end position
    let (start, end) = (end, start);
    let mut queue = VecDeque::new();
    queue.push_front((start, 0));

    while let Some((position, distance)) = queue.pop_front() {
        if let GridTile::Open(distances) = &mut tiles[position] {
            if distances.1.is_none() {
                distances.1 = Some(distance);
                [
                    position + 1,
                    position - 1,
                    position + width,
                    position - width,
                ]
                .into_iter()
                .for_each(|new_position| queue.push_back((new_position, distance + 1)));
            }
        }

        if position == end {
            break;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub tiles: Vec<GridTile>,
    pub width: usize,
    pub height: usize,
    pub original_distance: usize,
}

impl Map {
    fn parse_input(input: &str) -> Self {
        let width = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>()
            .len();
        let height = input.lines().collect::<Vec<_>>().len();
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut tiles: Vec<GridTile> = input
            .chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .map(|(n, c)| match c {
                '#' => GridTile::Wall,
                '.' => GridTile::Open(Distances(None, None)),
                'S' => {
                    start = Some(n);
                    GridTile::Open(Distances(None, None))
                }
                'E' => {
                    end = Some(n);
                    GridTile::Open(Distances(None, None))
                }
                _ => unreachable!("Unrecognized character: {}", c),
            })
            .collect();

        let start = start.expect("No start value found");
        let end = end.expect("No end value found");

        find_original_min_distances(&mut tiles, width, start, end);

        let best_distance = match tiles[end] {
            GridTile::Open(Distances(Some(dist), _)) => dist,
            _ => unreachable!("No path reached the goal"),
        };

        Map {
            tiles,
            height,
            width,
            original_distance: best_distance,
        }
    }

    fn len(&self) -> usize {
        self.tiles.len()
    }

    fn check_for_cheats(&self, x: usize, y: usize, save_distance: usize) -> Option<usize> {
        let tiles = &self.tiles;
        let position = self.width * y + x;
        if let GridTile::Wall = &tiles[position] {
            let neighbors = [
                &tiles[position + 1],
                &tiles[position - 1],
                &tiles[position + self.width],
                &tiles[position - self.width],
            ];

            Some(
                neighbors
                    .iter()
                    .flat_map(|a| {
                        neighbors.iter().filter(move |b| {
                            if let (
                                GridTile::Open(Distances(Some(start_distance), _)),
                                GridTile::Open(Distances(_, Some(end_distance))),
                            ) = (*a, **b)
                            {
                                start_distance + end_distance + 2
                                    <= self.original_distance - save_distance
                            } else {
                                false
                            }
                        })
                    })
                    .count(),
            )
        } else {
            None
        }
    }
    fn cheat_count(&self) -> usize {
        let minimum_cheat_distance = if self.width > 20 { 100 } else { 2 };

        let end_position = (self.len() / self.width) - 1;
        (1..end_position)
            .flat_map(|y| {
                let end_pos = self.width - 1;
                (1..end_pos)
                    .map(move |x| (x, y, minimum_cheat_distance))
                    .filter_map(|(x, y, save_distance)| self.check_for_cheats(x, y, save_distance))
            })
            .sum()
    }

    pub fn find_cheats_from_position(
        &self,
        x: usize,
        y: usize,
        cheat_distance: usize,
        save_distance: usize,
    ) -> Option<usize> {
        let pos = y * self.width + x;
        if let GridTile::Open(Distances(Some(distance_from_start), _)) = self.tiles[pos] {
            let check_start = x.max(cheat_distance + 1) - cheat_distance;
            let check_end = (x + cheat_distance).min(self.width - 2);
            Some(
                (check_start..=check_end)
                    .flat_map(|x_offset| {
                        let x_distance = x.abs_diff(x_offset);
                        let max_y_distance = cheat_distance - x_distance;
                        let check_start = y.max(max_y_distance + 1) - max_y_distance;
                        let check_end = (y + max_y_distance).min(self.height - 2);
                        (check_start..=check_end).filter(move |y_offset| {
                            let other_position = *y_offset * self.width + x_offset;
                            if let GridTile::Open(Distances(_, Some(distance_from_end))) =
                                self.tiles[other_position]
                            {
                                distance_from_start
                                    + distance_from_end
                                    + x_distance
                                    + y.abs_diff(*y_offset)
                                    <= self.original_distance - save_distance
                            } else {
                                false
                            }
                        })
                    })
                    .count(),
            )
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::parse_input(input);

    Some(map.cheat_count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::parse_input(input);
    let save_distance = if map.width < 20 { 50 } else { 100 };
    let max_cheat = 20;
    let height = map.height - 1;

    let result = (1..height)
        .flat_map(|y| {
            (1..map.width - 1)
                .map(move |x| (x, y))
                .filter_map(|(x, y)| map.find_cheats_from_position(x, y, max_cheat, save_distance))
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
