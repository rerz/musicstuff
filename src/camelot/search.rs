use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter;
use std::sync::LazyLock;

use petgraph::{Graph, Undirected};
use petgraph::prelude::{EdgeRef, NodeIndex};

use graphstuff::algo::clique::bron_kerbosch;
use graphstuff::graph::SimpleGraph;

use crate::camelot::{Key, make_standard_scale};
use crate::camelot::transition::{harmonic_transitions, KeyTransition, make_transition};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct NodeDistance {
    node: NodeIndex,
    distance: usize,
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Path {
    cost: i32,
    node: NodeIndex<u32>,
    transition: Option<KeyTransition>,
    pub path: Vec<NodeIndex<u32>>,
    transition_path: Vec<KeyTransition>,
}

impl Eq for Path {}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        self.cost == other.cost
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Path) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Creates a graph representing the possible harmonic scale transitions in the camelot wheel
pub fn make_scale_transition_graph() -> ScaleTransitions {
    
    let mut graph = petgraph::Graph::<Key, KeyTransition, Undirected>::new_undirected();

    let nodes = make_standard_scale();
    let transitions = harmonic_transitions();

    let scale_to_index = nodes
        .iter()
        .map(|scale| (*scale, graph.add_node(*scale)))
        .collect::<HashMap<_, _>>();

    for scale in &nodes {
        let source_scale_node = *scale_to_index.get(scale).unwrap();
        for transition in &transitions {
            let target_scale = make_transition(*scale, *transition);
            let target_scale_node = *scale_to_index.get(&target_scale).unwrap();
            graph.add_edge(source_scale_node, target_scale_node, *transition);
        }
    }

    ScaleTransitions {
        graph,
        index: scale_to_index,
    }
}

#[test]
fn test_cliques() {
    dbg!(SCALE_TRANSITION_GRAPH.cliques());
}

pub struct ScaleTransitions {
    index: HashMap<Key, NodeIndex>,
    graph: Graph<Key, KeyTransition, Undirected>,
}

impl ScaleTransitions {
    /// Returns the maximal cliques in the scale transition graph (only triangles)
    pub fn cliques(&self) -> Vec<HashSet<u32>> {
        bron_kerbosch(&SimpleGraph::from(self.graph.clone()))
    }

    pub fn path(source: Key, target: Key) -> Vec<Key> {
        let graph = &SCALE_TRANSITION_GRAPH.graph;
        let index = &SCALE_TRANSITION_GRAPH.index;

        let source_idx = index.get(&source).unwrap();
        let target_idx = index.get(&target).unwrap();

        let paths = multi_path_dijkstra(graph, *source_idx, *target_idx, 1);
        let path = paths.into_iter().nth(0).unwrap();

        let path = path
            .path
            .into_iter()
            .map(|node| *graph.node_weight(node).unwrap())
            .collect::<Vec<_>>();

        path
    }
}

pub static SCALE_TRANSITION_GRAPH: LazyLock<ScaleTransitions> =
    LazyLock::new(|| make_scale_transition_graph());

/// Implementation of dijkstra's algorithm that returns the top n shortest paths
fn multi_path_dijkstra(
    graph: &Graph<Key, KeyTransition, Undirected>,
    source: NodeIndex<u32>,
    target: NodeIndex<u32>,
    n: usize,
) -> Vec<Path> {
    let mut min_heap = BinaryHeap::new();
    let mut paths = Vec::new();

    min_heap.push(Path {
        cost: 0,
        node: source,
        transition: None,
        path: vec![],
        transition_path: vec![],
    });

    while let Some(mut path) = min_heap.pop() {
        path.path.push(path.node);

        if let Some(transition) = path.transition {
            path.transition_path.push(transition);
        }

        if path.node == target {
            paths.push(path.clone());
            if paths.len() >= n {
                break;
            }
        }

        for edge in graph.edges(path.node) {
            let neighbor = edge.target();
            let weight = graph.edge_weight(edge.id()).unwrap();
            min_heap.push(Path {
                cost: path.cost + 1,
                node: neighbor,
                transition: Some(*weight),
                transition_path: path.transition_path.clone(),
                path: path.path.clone(),
            });
        }
    }

    paths
}

/// Returns all possible harmonic transitions from a given key
pub fn harmonic_transitions_from(scale: Key) -> Vec<Key> {
    let mut harmonic = harmonic_transitions()
        .map(|transition| make_transition(scale, transition))
        .into_iter()
        .chain(iter::once(scale))
        .collect::<Vec<_>>();

    harmonic.sort();

    harmonic
}
