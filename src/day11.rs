use std::collections::HashMap;
use std::iter::once;
use std::str::FromStr;
use std::string::ToString;

use itertools::Itertools;

use crate::graph::Graph;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
}

const INPUT: &str = include_str!("../input/input.day11");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 1 hour 18 minutes 42,78 seconds
    let connections = Connections::from_str(input).expect("Should parse");
    let mut graph = Graph::new();
    let names_with_ids = connections
        .0
        .iter()
        .flat_map(|connection| once(&connection.device).chain(&connection.output))
        .unique()
        .map(|name| (name, graph.add_node((*name).clone())))
        .collect::<HashMap<_, _>>();
    for connection in &connections.0 {
        let from = names_with_ids
            .get(&connection.device)
            .expect("Should contain device name as added beforehand");
        for output in &connection.output {
            let to = names_with_ids
                .get(output)
                .expect("Should contain device name as added beforehand");
            graph
                .add_edge(*from, *to, 1)
                .expect("Should add edge as device names exist in graph")
        }
    }
    let start = names_with_ids
        .get(&DeviceName(START.to_string()))
        .expect("Should contain start device");
    graph.calculate(*start, None);

    let end = names_with_ids
        .get(&DeviceName(END.to_string()))
        .expect("Should contain end device");

    graph
        .node(*end)
        .expect("Should get end node")
        .borrow()
        .distance_from_start()
        .len() as u64
}

mod graph {
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashSet};
    use std::rc::Rc;

    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct Graph<V> {
        nodes: Vec<Rc<RefCell<Node<V>>>>,
        next_node_index: NodeId,
        edges: HashSet<Edge>,
    }

    impl<V> Graph<V> {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                next_node_index: NodeId(0),
                edges: HashSet::new(),
            }
        }
        pub fn add_node(&mut self, value: V) -> NodeId {
            let id = self.next_node_index;
            self.next_node_index = NodeId(self.next_node_index.0 + 1);
            self.nodes.push(Rc::new(RefCell::new(Node {
                id,
                _value: value,
                distance_from_start: Vec::new(),
            })));
            id
        }

        pub fn node(&self, id: NodeId) -> Option<Rc<RefCell<Node<V>>>> {
            self.nodes
                .iter()
                .find(|node| node.borrow().id == id)
                .cloned()
        }

        pub fn add_edge(
            &mut self,
            from: NodeId,
            to: NodeId,
            weight: u64,
        ) -> Result<(), AddEdgeError> {
            if !self.contains(from) {
                Err(AddEdgeError::NodeNotFound(from))
            } else if !self.contains(to) {
                Err(AddEdgeError::NodeNotFound(to))
            } else if !self.edges.insert(Edge { from, to, weight }) {
                Err(AddEdgeError::AlreadyExists)
            } else {
                Ok(())
            }
        }

        pub fn contains(&self, id: NodeId) -> bool {
            self.nodes.iter().any(|node| node.borrow().id == id)
        }

        pub fn calculate(&mut self, start: NodeId, end: Option<NodeId>) {
            let mut next = BinaryHeap::new();
            let mut visited = Vec::new();

            for node in &mut self.nodes {
                node.borrow_mut().distance_from_start.clear();
                if node.borrow().id == start {
                    node.borrow_mut().distance_from_start.push((start, 0));
                    next.push(Rc::clone(node));
                }
            }

            while let Some(current) = next.pop() {
                // Only checking nodes reachable by start
                if current.borrow().distance_from_start.is_empty() {
                    break;
                }

                // If found the end, terminate
                if let Some(end) = end
                    && end == current.borrow().id
                {
                    break;
                }

                visited.push(Rc::clone(&current));

                let outgoing_edges = self
                    .edges
                    .iter()
                    .filter(|edge| edge.from == current.borrow().id)
                    .collect::<Box<[_]>>();
                for edge in outgoing_edges {
                    let neighbour = self
                        .nodes
                        .iter()
                        .find(|node| node.borrow().id == edge.to)
                        .expect("Should find node");
                    neighbour.borrow_mut().distance_from_start.push((
                        edge.from,
                        current
                            .borrow()
                            .minimum_distance()
                            .expect("Should have minimum distance")
                            .1
                            + 1,
                    ));
                    if !visited.contains(neighbour) {
                        next.push(Rc::clone(neighbour));
                    }
                }
            }
        }
    }

    #[derive(thiserror::Error, Debug)]
    pub enum AddEdgeError {
        #[error("Did not find node with id '{0}'")]
        NodeNotFound(NodeId),
        #[error("Edge already exists")]
        AlreadyExists,
    }

    #[derive(Clone, Debug)]
    pub struct Node<V> {
        id: NodeId,
        _value: V,
        distance_from_start: Vec<(NodeId, u64)>,
    }

    impl<V> Node<V> {
        pub fn distance_from_start(&self) -> &[(NodeId, u64)] {
            &self.distance_from_start
        }

        pub fn minimum_distance(&self) -> Option<&(NodeId, u64)> {
            self.distance_from_start
                .iter()
                .max_by_key(|(_, distance)| *distance)
        }
    }

    impl<V> Eq for Node<V> {}

    impl<V> PartialEq for Node<V> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl<V> Ord for Node<V> {
        fn cmp(&self, other: &Self) -> Ordering {
            let self_distance = self.minimum_distance().map(|(_, distance)| *distance);
            let other_distance = other.minimum_distance().map(|(_, distance)| *distance);
            match (self_distance, other_distance) {
                (Some(self_distance), Some(other_distance)) => {
                    // Reversed to have MinHeap
                    other_distance.cmp(&self_distance)
                }
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (None, None) => Ordering::Equal,
            }
        }
    }

    impl<V> PartialOrd for Node<V> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(derive_more::Display, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub struct NodeId(u64);

    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    struct Edge {
        from: NodeId,
        to: NodeId,
        weight: u64,
    }
}

const START: &str = "you";
const END: &str = "out";

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Connections(Box<[Connection]>);

impl FromStr for Connections {
    type Err = ParseConnectionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseConnectionsError {
    #[error("Failed to parse connection: {0}")]
    ParseConnection(#[from] ParseConnectionError),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Connection {
    device: DeviceName,
    output: Box<[DeviceName]>,
}

impl FromStr for Connection {
    type Err = ParseConnectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (device_name_str, outputs_str) = s
            .split_once(": ")
            .ok_or(ParseConnectionError::MissingDelimiter)?;
        Ok(Self {
            device: device_name_str.parse()?,
            output: outputs_str
                .split_whitespace()
                .map(|output_str| output_str.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseConnectionError {
    #[error("Missing delimiter ': '")]
    MissingDelimiter,
    #[error("Failed to parse device name: {0}")]
    ParseDeviceName(#[from] ParseDeviceNameError),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct DeviceName(String);

impl FromStr for DeviceName {
    type Err = ParseDeviceNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseDeviceNameError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = include_str!("../input/example.day11");

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 5);
    }
}
