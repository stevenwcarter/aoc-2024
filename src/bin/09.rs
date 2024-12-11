use std::collections::VecDeque;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    File(usize),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileBlock {
    size: usize,
    id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block2 {
    File(FileBlock),
    Empty(usize),
}

#[derive(Default, Debug)]
pub struct Memory {
    pub memory: VecDeque<Block>,
}

#[derive(Default, Debug)]
pub struct Memory2 {
    pub memory: Vec<Block2>,
}

// memory for part 2
impl Memory2 {
    pub fn push_file(&mut self, size: usize, id: usize) {
        self.memory.push(Block2::File(FileBlock { size, id }));
    }
    pub fn push_freespace(&mut self, size: usize) {
        self.memory.push(Block2::Empty(size));
    }
    pub fn use_freespace_at_idx(&mut self, index: usize, size: usize, id: usize) {
        let free_space = self.memory.get_mut(index).unwrap();
        if let Block2::Empty(empty_size) = free_space {
            let empty_size = *empty_size;
            if empty_size == size {
                *free_space = Block2::File(FileBlock { size, id });
            } else {
                *free_space = Block2::Empty(empty_size - size);
                self.memory.splice(
                    index..index,
                    std::iter::once(Block2::File(FileBlock { size, id })),
                );
            }
        }
    }
    pub fn get_next_block_to_move(&self, last_index: usize) -> Option<(usize, FileBlock)> {
        self.memory[0..last_index]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, b)| {
                if let Block2::File(file_block) = b {
                    return Some((index, *file_block));
                }
                None
            })
    }
    pub fn find_free_space(
        &self,
        needed_size: usize,
        source_index: usize,
    ) -> Option<(usize, usize)> {
        self.memory[0..source_index]
            .iter()
            .enumerate()
            .find_map(|(index, b)| {
                if let Block2::Empty(empty_block_size) = b {
                    if needed_size <= *empty_block_size {
                        return Some((index, *empty_block_size));
                    }
                }
                None
            })
    }
    pub fn compact(&mut self) {
        let mut stop = false;

        // speeds up the processing by starting closer to the right spot
        let mut last_index = self.memory.len();

        while !stop {
            if let Some((source_index, block_to_move)) = self.get_next_block_to_move(last_index) {
                last_index = source_index;
                if let Some((free_index, _free_size)) =
                    self.find_free_space(block_to_move.size, source_index)
                {
                    if free_index < source_index {
                        let original = self.memory.get_mut(source_index).unwrap();
                        *original = Block2::Empty(block_to_move.size);
                        self.use_freespace_at_idx(free_index, block_to_move.size, block_to_move.id);
                    }
                }
            } else {
                stop = true;
            }
        }
    }

    pub fn checksum(&self) -> usize {
        let mut checksum_vec: Vec<usize> = Vec::new();
        self.memory.iter().for_each(|b| match b {
            Block2::File(file_block) => {
                (0..file_block.size).for_each(|_| {
                    checksum_vec.push(file_block.id);
                });
            }
            Block2::Empty(size) => {
                (0..*size).for_each(|_| {
                    checksum_vec.push(0);
                });
            }
        });

        checksum_vec
            .iter()
            .enumerate()
            .map(|(pos, id)| pos * id)
            .sum()
    }
    pub fn print(&self) {
        let msg = self
            .memory
            .iter()
            .map(|b| match b {
                Block2::File(file_block) => (0..file_block.size)
                    .map(|_| file_block.id.to_string())
                    .collect::<String>(),
                Block2::Empty(size) => (0..*size).map(|_| ".".to_string()).collect::<String>(),
            })
            .collect::<String>();

        println!("{}", msg);
    }
}

impl Memory {
    pub fn push_file(&mut self, size: usize, id: usize) {
        (0..size).for_each(|_| {
            self.memory.push_back(Block::File(id));
        });
    }
    pub fn push_freespace(&mut self, size: usize) {
        (0..size).for_each(|_| {
            self.memory.push_back(Block::Empty);
        });
    }
    pub fn find_index_of_first_gap(&self) -> Option<usize> {
        self.memory.iter().enumerate().find_map(|(idx, b)| {
            if matches!(b, Block::Empty) {
                Some(idx)
            } else {
                None
            }
        })
    }

    pub fn replace_at_index(&mut self, idx: usize, b: Block) {
        let old_element = self.memory.get_mut(idx).unwrap();
        *old_element = b;
    }

    pub fn compact(&mut self) {
        let mut free_space: VecDeque<usize> = self
            .memory
            .iter()
            .enumerate()
            .filter_map(|(idx, b)| {
                if let Block::Empty = b {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();
        let blocks_to_move: VecDeque<(usize, usize)> = self
            .memory
            .iter()
            .enumerate()
            .filter_map(|(idx, b)| {
                if let Block::File(id) = b {
                    Some((idx, *id))
                } else {
                    None
                }
            })
            .collect();

        let mut stop = false;
        blocks_to_move.iter().rev().for_each(|(idx, id)| {
            if !stop {
                let index_to_update = free_space.pop_front();
                if let Some(index_to_update) = index_to_update {
                    if index_to_update < *idx {
                        self.replace_at_index(index_to_update, Block::File(*id));
                        *self.memory.get_mut(*idx).unwrap() = Block::Empty;
                    } else {
                        stop = true;
                    }
                } else {
                    stop = true;
                }
            }
        });
    }

    pub fn checksum(&self) -> usize {
        self.memory
            .iter()
            .enumerate()
            .map(|(pos, b)| {
                pos * match b {
                    Block::File(size) => *size,
                    _ => 0,
                }
            })
            .sum()
    }

    pub fn print(&self) {
        let msg = self
            .memory
            .iter()
            .map(|b| match b {
                Block::File(id) => id.to_string(),
                Block::Empty => ".".to_string(),
            })
            .collect::<String>();

        println!("{}", msg);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut is_next_file = true;
    let mut id = 0;
    let mut memory = Memory::default();

    for char in input.trim_end().chars() {
        let size = char.to_digit(10).unwrap() as usize;
        if is_next_file {
            memory.push_file(size, id);
            id += 1;
        } else {
            memory.push_freespace(size);
        }
        is_next_file = !is_next_file;
    }

    memory.compact();

    Some(memory.checksum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut is_next_file = true;
    let mut id = 0;
    let mut memory = Memory2::default();

    for char in input.trim_end().chars() {
        let size = char.to_digit(10).unwrap() as usize;
        if is_next_file {
            // file
            memory.push_file(size, id);
            id += 1;
        } else {
            // free space
            memory.push_freespace(size);
        }
        is_next_file = !is_next_file;
    }

    memory.compact();

    Some(memory.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
