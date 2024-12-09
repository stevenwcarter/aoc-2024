use std::collections::VecDeque;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    File(usize),
    Empty,
}

#[derive(Default, Debug)]
pub struct Memory {
    pub memory: VecDeque<Block>,
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

    // TODO: speed this up by building an ordered vec of all the ids at the end, then
    // loop through all free-space slots, replacing them with ids from the end, and removing from
    // the back each time something is successfully moved.
    pub fn compact(&mut self) {
        // let mut free_space: VecDeque<usize> = self
        //     .memory
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(idx, b)| {
        //         if let Block::Empty = b {
        //             Some(idx)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();

        let mut stop = false;

        while !stop {
            let block_to_move = self.memory.pop_back();
            if let Some(Block::File(_)) = block_to_move {
                let index_to_swap = self.find_index_of_first_gap();
                match index_to_swap {
                    Some(idx) => {
                        self.replace_at_index(idx, block_to_move.unwrap());
                    }
                    None => {
                        self.memory.push_back(block_to_move.unwrap());
                        stop = true;
                    }
                }
            }
        }
    }

    // TODO: Speed this up by changing how I map memory. Write it out for the checksum, but
    // otherwise this can be more directly mapped for improved speed. Could just by a vec of
    // (block_type, size), and account for placing a smaller block into a larger free space block,
    // prepending with the new block and changing the size of the free space block to match.
    pub fn compact_part_two(&mut self) {
        let mut files = Vec::new();
        let mut current_file_id = None;
        let mut start_idx = 0;

        for (idx, block) in self.memory.iter().enumerate() {
            match block {
                Block::File(file_id) => {
                    if current_file_id.is_none() || current_file_id.unwrap() != *file_id {
                        // Found the start of a new file
                        if let Some(id) = current_file_id {
                            files.push((id, start_idx, idx - start_idx));
                        }
                        current_file_id = Some(*file_id);
                        start_idx = idx;
                    }
                }
                Block::Empty => {
                    if let Some(id) = current_file_id {
                        files.push((id, start_idx, idx - start_idx));
                        current_file_id = None;
                    }
                }
            }
        }
        // handle the last file
        if let Some(id) = current_file_id {
            files.push((id, start_idx, self.memory.len() - start_idx));
        }

        // sort by file id order
        files.sort_by(|a, b| b.0.cmp(&a.0));

        for (file_id, current_start_idx, size) in files {
            // find the first free space span large enough
            let mut free_start = None;
            let mut free_size = 0;

            for (idx, block) in self.memory.iter().enumerate() {
                match block {
                    Block::Empty => {
                        if free_start.is_none() {
                            free_start = Some(idx);
                        }
                        free_size += 1;
                    }
                    _ => {
                        free_start = None;
                        free_size = 0;
                    }
                }

                if free_size == size {
                    break;
                }
            }

            // confirm this span is valid
            if let Some(free_start_idx) = free_start {
                if free_size >= size && free_start_idx < current_start_idx {
                    // move the file to the free span
                    for i in 0..size {
                        self.memory[free_start_idx + i] = Block::File(file_id);
                    }
                    // clear the original storage location
                    for i in 0..size {
                        self.memory[current_start_idx + i] = Block::Empty;
                    }
                }
            }
        }
    }

    pub fn checksum(&self) -> usize {
        self.memory
            .iter()
            .enumerate()
            .filter(|(_, b)| matches!(b, Block::File(_)))
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
    // memory.print();

    Some(memory.checksum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut is_next_file = true;
    let mut id = 0;
    let mut memory = Memory::default();

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

    // memory.print();
    memory.compact_part_two();

    // memory.print();
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
        // println!("00992111777.44.333....5555.6666.....8888..");
        assert_eq!(result, Some(2858));
    }
}
