advent_of_code::solution!(5);

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::str::FromStr;

use std::collections::{HashMap, HashSet};

fn dfs_topological_sort(
    node: u32,
    graph: &HashMap<u32, Vec<u32>>,
    visited: &mut HashSet<u32>,
    rec_stack: &mut HashSet<u32>,
    stack: &mut Vec<u32>,
) -> Result<(), &'static str> {
    if visited.contains(&node) {
        return Ok(());
    }

    visited.insert(node);
    rec_stack.insert(node);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            dfs_topological_sort(neighbor, graph, visited, rec_stack, stack)?;
        }
    }

    rec_stack.remove(&node);
    stack.push(node);

    Ok(())
}

fn topological_sort_dfs(rules: &[(u32, u32)], data: &[u32]) -> Result<Vec<u32>, &'static str> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    rules
        .iter()
        .filter(|(a, b)| data.contains(a) && data.contains(b))
        .for_each(|(a, b)| {
            graph.entry(*a).or_default().push(*b);
        });

    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut stack = Vec::new();

    for &node in graph.keys() {
        if !visited.contains(&node) {
            dfs_topological_sort(node, &graph, &mut visited, &mut rec_stack, &mut stack)?;
        }
    }

    stack.reverse();
    Ok(stack)
}

fn reorder_sequence(global_order: &[u32], sequence: Vec<u32>) -> Vec<u32> {
    let order_map: HashMap<u32, usize> = global_order
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();

    // Filter and sort the sequence based on the global order
    let mut filtered_sequence: Vec<u32> = sequence
        .into_iter()
        .filter(|&num| order_map.contains_key(&num))
        .collect();

    filtered_sequence.sort_by_key(|&num| order_map[&num]);
    filtered_sequence
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| u32::from_str(s))(input)
}

fn parse_pipe_line(input: &str) -> IResult<&str, (u32, u32)> {
    terminated(
        separated_pair(parse_number, tag("|"), parse_number),
        line_ending,
    )(input)
}

fn parse_comma_line(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(separated_list1(tag(","), parse_number), line_ending)(input)
}

type Rules = Vec<(u32, u32)>;
type Data = Vec<Vec<u32>>;

fn parse_input(input: &str) -> IResult<&str, (Rules, Data)> {
    let (input, pipe_lines) = many1(parse_pipe_line)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, comma_lines) = many1(parse_comma_line)(input)?;
    Ok((input, (pipe_lines, comma_lines)))
}

fn is_correct_order(data: &[u32], checks: &[(u32, u32)]) -> bool {
    checks.iter().all(|(l, r)| {
        let lr = data.iter().position(|v| v == l);
        let rr = data.iter().position(|v| v == r);
        match (lr, rr) {
            (Some(lr), Some(rr)) => lr < rr,
            _ => true,
        }
    })
}

fn find_middle(data: &[u32]) -> u32 {
    let mid_index = data.len() / 2;
    data[mid_index]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (checks, data) = parse_input(input).unwrap().1;
    Some(
        data.iter()
            .filter(|d| is_correct_order(d, &checks))
            .map(|d| find_middle(d))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (checks, data) = parse_input(input).unwrap().1;

    Some(
        data[..]
            .iter()
            .filter(|d| !is_correct_order(d, &checks))
            .map(|d| {
                let correct_order = topological_sort_dfs(&checks, d).unwrap();
                reorder_sequence(&correct_order, d.to_vec())
            })
            .map(|d| find_middle(&d))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
