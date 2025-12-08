// --- Day 8: Playground ---
// https://adventofcode.com/2025/day/8

use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::unionfind::UnionFind;
use petgraph::visit::Dfs;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Coordinate) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        (dx.pow(2) + dy.pow(2) + dz.pow(2)).isqrt()
    }
}

pub fn create_sorted_pairs(input: &[Coordinate]) -> Vec<((Coordinate, Coordinate), usize)> {
    let mut pairs: Vec<_> = input
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| ((*a, *b), a.distance(b)))
        .collect();

    pairs.sort_by_key(|(_, distance)| *distance);
    pairs
}

fn init_graph(
    input: &[Coordinate],
) -> (UnGraph<Coordinate, usize>, HashMap<Coordinate, NodeIndex>) {
    let mut graph = UnGraph::<Coordinate, usize>::new_undirected();
    let mut coordinate_to_node = HashMap::new();

    for coordinate in input.iter() {
        let node_idx = graph.add_node(*coordinate);
        coordinate_to_node.insert(*coordinate, node_idx);
    }

    (graph, coordinate_to_node)
}

pub fn create_graph(
    input: &[Coordinate],
    sorted_pairs: &[((Coordinate, Coordinate), usize)],
    connect_limit: usize,
) -> UnGraph<Coordinate, usize> {
    let (mut graph, coordinate_to_node) = init_graph(input);
    let mut uf = UnionFind::<NodeIndex>::new(input.len());

    for ((coord1, coord2), _) in sorted_pairs.iter().take(connect_limit) {
        let node1 = coordinate_to_node[coord1];
        let node2 = coordinate_to_node[coord2];

        if uf.union(node1, node2) {
            graph.add_edge(node1, node2, 0);
        }
    }

    graph
}

pub fn create_connected_graph_with_union_find(
    input: &[Coordinate],
    sorted_pairs: &[((Coordinate, Coordinate), usize)],
) -> (UnGraph<Coordinate, usize>, Option<(Coordinate, Coordinate)>) {
    let (mut graph, coordinate_to_node) = init_graph(input);
    let mut uf = UnionFind::<NodeIndex>::new(input.len());
    let mut last_edge = None;
    let mut num_components = input.len();

    for ((coord1, coord2), _) in sorted_pairs {
        let node1 = coordinate_to_node[coord1];
        let node2 = coordinate_to_node[coord2];

        if uf.union(node1, node2) {
            last_edge = Some((*coord1, *coord2));
            graph.add_edge(node1, node2, 0);

            num_components -= 1;

            if num_components == 1 {
                break;
            }
        }
    }

    (graph, last_edge)
}

pub fn get_idxs_of_connected_components(graph: &UnGraph<Coordinate, usize>) -> Vec<Vec<NodeIndex>> {
    let mut visited: HashSet<NodeIndex> = HashSet::new();
    let mut components: Vec<Vec<NodeIndex>> = Vec::new();

    for node_idx in graph.node_indices() {
        if visited.contains(&node_idx) {
            continue;
        }

        let mut component = Vec::new();
        let mut dfs = Dfs::new(&graph, node_idx);

        while let Some(next) = dfs.next(&graph) {
            if visited.insert(next) {
                component.push(next);
            }
        }

        if !component.is_empty() {
            components.push(component);
        }
    }

    components.sort_by_key(|component| component.len());
    components.reverse();

    components
}

pub fn product_of_lengths_of_connected_components(
    components: &[Vec<NodeIndex>],
    n: usize,
) -> usize {
    components
        .iter()
        .take(n)
        .map(|component| component.len())
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::Coordinate;
    use pretty_assertions::assert_eq;

    use std::fs;

    #[allow(unused)]
    fn file_to_vec(file: &str) -> Vec<Coordinate> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| {
                let parts: Vec<usize> = s
                    .split(',')
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect();
                Coordinate::new(parts[0], parts[1], parts[2])
            })
            .collect::<Vec<Coordinate>>()
    }

    #[test]
    fn test_product_of_connected_components() {
        let input = file_to_vec("../input/day08.txt");
        let sorted_pairs = super::create_sorted_pairs(&input);
        let graph = super::create_graph(&input, &sorted_pairs, 1000);
        let components = super::get_idxs_of_connected_components(&graph);

        assert_eq!(
            super::product_of_lengths_of_connected_components(&components, 3),
            90036
        );
    }

    #[test]
    fn test_union_find() {
        let input = file_to_vec("../input/day08.txt");
        let sorted_pairs = super::create_sorted_pairs(&input);
        let (_graph, last_edge) =
            super::create_connected_graph_with_union_find(&input, &sorted_pairs);

        if let Some((coord1, coord2)) = last_edge {
            assert_eq!(coord1.x * coord2.x, 6083499488);
        }
    }
}
