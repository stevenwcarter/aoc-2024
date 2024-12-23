advent_of_code::solution!(23);
use hashbrown::{HashMap, HashSet};
use std::collections::BTreeSet;

#[derive(Debug)]
struct Node {
    pub name: String,
    pub edges: Vec<usize>,
}

#[derive(Debug, Default)]
struct Graph {
    pub nodes: Vec<Node>,
    pub seen: HashMap<String, usize>,
}

impl Graph {
    fn add_node(&mut self, name: &str) -> usize {
        if let Some(&id) = self.seen.get(name) {
            return id;
        }
        let id = self.nodes.len();
        self.seen.insert(name.to_string(), id);
        self.nodes.push(Node {
            name: name.to_string(),
            edges: vec![],
        });
        id
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        let a_id = self.add_node(a);
        let b_id = self.add_node(b);
        self.nodes[a_id].edges.push(b_id);
        self.nodes[b_id].edges.push(a_id);
    }

    pub fn largest_clique_sorted(&self) -> Vec<String> {
        let mut max_clique = Vec::new();
        let mut potential_clique = HashSet::new();
        let mut candidates: HashSet<_> = (0..self.nodes.len()).collect();
        let mut excluded = HashSet::new();

        self.bron_kerbosch(
            &mut potential_clique,
            &mut candidates,
            &mut excluded,
            &mut max_clique,
        );

        let mut result: Vec<String> = max_clique
            .iter()
            .map(|&i| self.nodes[i].name.clone())
            .collect();
        result.sort();

        result
    }

    fn bron_kerbosch(
        &self,
        potential_clique: &mut HashSet<usize>,
        candidates: &mut HashSet<usize>,
        excluded: &mut HashSet<usize>,
        max_clique: &mut Vec<usize>,
    ) {
        if candidates.is_empty() && excluded.is_empty() {
            if potential_clique.len() > max_clique.len() {
                max_clique.clear();
                max_clique.extend(potential_clique.iter());
            }
            return;
        }

        let pivot = self.choose_pivot(candidates, excluded);

        let candidates_without_pivot_neighbors = candidates
            .difference(&self.nodes[pivot].edges.iter().copied().collect())
            .cloned()
            .collect::<Vec<_>>();

        for &node in &candidates_without_pivot_neighbors {
            potential_clique.insert(node);
            let neighbors: HashSet<_> = self.nodes[node].edges.iter().copied().collect();

            let mut new_candidates = candidates.intersection(&neighbors).cloned().collect();
            let mut new_excluded = excluded.intersection(&neighbors).cloned().collect();

            self.bron_kerbosch(
                potential_clique,
                &mut new_candidates,
                &mut new_excluded,
                max_clique,
            );

            potential_clique.remove(&node);
            candidates.remove(&node);
            excluded.insert(node);
        }
    }

    fn choose_pivot(&self, candidates: &HashSet<usize>, excluded: &HashSet<usize>) -> usize {
        let union: HashSet<_> = candidates.union(excluded).cloned().collect();
        union
            .into_iter()
            .max_by_key(|&node| self.nodes[node].edges.len())
            .unwrap()
    }
}

type Cycle = BTreeSet<usize>;

// recursively explore the graph, finding all groupings that exist up
// to the depth selected
fn find_cycles(
    g: &Graph,
    start: usize,
    cur: usize,
    seen: &mut BTreeSet<usize>,
    depth: usize,
    found: &mut HashSet<Cycle>,
) {
    seen.insert(cur);

    if depth == 0 {
        if g.nodes[cur].edges.contains(&start) {
            // store the new grouping
            let cycle = seen.clone();
            found.insert(cycle);
        }
        seen.remove(&cur);
        return;
    }

    for &next in &g.nodes[cur].edges {
        if seen.contains(&next) {
            continue;
        }
        find_cycles(g, start, next, seen, depth - 1, found);
    }

    seen.remove(&cur);
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cycles = HashSet::new();
    let mut graph = Graph::default();
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(a, b)| graph.add_edge(a, b));

    graph
        .seen
        .iter()
        .filter(|(name, _)| name.starts_with('t'))
        .for_each(|(_, &id)| find_cycles(&graph, id, id, &mut BTreeSet::new(), 2, &mut cycles));

    Some(cycles.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut graph = Graph::default();
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(a, b)| {
            graph.add_edge(a, b);
        });

    Some(graph.largest_clique_sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
