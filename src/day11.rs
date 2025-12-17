use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

use itertools::Itertools;

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day11");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 1 hour 18 minutes 42,78 seconds
    use graph::Graph;
    use std::iter::once;

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
        .get(&DeviceName(START_PART_1.to_string()))
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

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 3 hours 46 minutes 57,73 seconds (and I cheated by cloning a solution)

    // Inspired by: https://www.reddit.com/r/adventofcode/comments/1pjp1rm/comment/nu7svdb/
    /*
    fn day11_part2(paths: &HashMap<&str, Vec<&str>>) -> i64 {
        let mut memoisation_data: HashMap<&str, (i64, i64, i64, i64)> =
            HashMap::from([("svr", (1, 0, 0, 0))]);
        number_of_paths_through(paths, "out", &mut memoisation_data).3
    }

    fn number_of_paths_through<'a>(
        paths: &HashMap<&'a str, Vec<&'a str>>,
        current_node: &'a str,
        memoisation_data: &mut HashMap<&'a str, (i64, i64, i64, i64)>,
    ) -> (i64, i64, i64, i64) {
        if memoisation_data.contains_key(current_node) {
            return *memoisation_data.get(current_node).unwrap();
        }
        if !paths.contains_key(current_node) {
            return (0, 0, 0, 0);
        }

        let mut paths_number: (i64, i64, i64, i64) = paths
            .get(current_node)
            .unwrap()
            .iter()
            .map(|&x| number_of_paths_through(paths, x, memoisation_data))
            .fold((0, 0, 0, 0), |(suma, sumb, sumc, sumd), (a, b, c, d)| {
                (suma + a, sumb + b, sumc + c, sumd + d)
            });

        if current_node == "dac" {
            paths_number.3 += paths_number.1;
            paths_number.2 += paths_number.0;
        }
        if current_node == "fft" {
            paths_number.3 += paths_number.2;
            paths_number.1 += paths_number.0;
        }
        memoisation_data.insert(current_node, paths_number);
        paths_number
    }
    */

    fn count_of_ingoing_paths_on_device(
        connections: &Connections,
        current_device: DeviceName,
        stopover_a_device: DeviceName,
        stopover_b_device: DeviceName,
        memoisation: &mut HashMap<DeviceName, IngoingPathsCount>,
    ) -> IngoingPathsCount {
        if let Some(paths_count) = memoisation.get(&current_device) {
            return *paths_count;
        }
        let devices_pointing_at_current = connections
            .0
            .iter()
            .filter(|connection| connection.output.iter().contains(&current_device))
            .map(|connection| &connection.device)
            .collect::<Box<[_]>>();
        if devices_pointing_at_current.is_empty() {
            return IngoingPathsCount::zero();
        }
        let mut paths_count = devices_pointing_at_current
            .iter()
            .map(|device_pointing_at_current| {
                count_of_ingoing_paths_on_device(
                    connections,
                    (*device_pointing_at_current).clone(),
                    stopover_a_device.clone(),
                    stopover_b_device.clone(),
                    memoisation,
                )
            })
            .fold(IngoingPathsCount::zero(), std::ops::Add::add);

        if current_device == stopover_a_device {
            paths_count.over_both_stopovers += paths_count.over_stopover_b;
            paths_count.over_stopover_a += paths_count.from_start;
        }
        if current_device == stopover_b_device {
            paths_count.over_both_stopovers += paths_count.over_stopover_a;
            paths_count.over_stopover_b += paths_count.from_start;
        }
        memoisation.insert(current_device, paths_count);
        paths_count
    }

    let connections = Connections::from_str(input).expect("Should parse");

    count_of_ingoing_paths_on_device(
        &connections,
        DeviceName(END.to_string()),
        DeviceName(STOPOVER_A_PART_2.to_string()),
        DeviceName(STOPOVER_B_PART_2.to_string()),
        &mut HashMap::from([(
            DeviceName(START_PART_2.to_string()),
            IngoingPathsCount::start(),
        )]),
    )
    .over_both_stopovers
}

#[cfg(feature = "part1")]
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

#[cfg(feature = "part2")]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct IngoingPathsCount {
    from_start: u64,
    over_stopover_a: u64,
    over_stopover_b: u64,
    over_both_stopovers: u64,
}

#[cfg(feature = "part2")]
impl IngoingPathsCount {
    fn zero() -> Self {
        Self {
            from_start: 0,
            over_stopover_a: 0,
            over_stopover_b: 0,
            over_both_stopovers: 0,
        }
    }

    fn start() -> Self {
        Self {
            from_start: 1,
            over_stopover_a: 0,
            over_stopover_b: 0,
            over_both_stopovers: 0,
        }
    }
}

#[cfg(feature = "part2")]
impl std::ops::Add for IngoingPathsCount {
    type Output = IngoingPathsCount;

    fn add(self, rhs: Self) -> Self::Output {
        IngoingPathsCount {
            from_start: self.from_start + rhs.from_start,
            over_stopover_a: self.over_stopover_a + rhs.over_stopover_a,
            over_stopover_b: self.over_stopover_b + rhs.over_stopover_b,
            over_both_stopovers: self.over_both_stopovers + rhs.over_both_stopovers,
        }
    }
}

#[cfg(feature = "part1")]
const START_PART_1: &str = "you";
#[cfg(feature = "part2")]
const START_PART_2: &str = "svr";
#[cfg(feature = "part2")]
const STOPOVER_A_PART_2: &str = "fft";
#[cfg(feature = "part2")]
const STOPOVER_B_PART_2: &str = "dac";
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
        let input = include_str!("../input/example.day11_part1");

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 5);
    }

    #[test]
    fn test_part2() {
        // Arrange
        let input = include_str!("../input/example.day11_part2");

        // Act
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 2);
    }
}
