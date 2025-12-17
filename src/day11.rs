use std::collections::{HashMap, VecDeque};
use std::iter::once;
use std::str::FromStr;
use std::string::ToString;

use itertools::Itertools;

use crate::graph::{Graph, NodeId};

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day11");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 1 hour 18 minutes 42,78 seconds
    let connections = Connections::from_str(input).expect("Should parse");
    let (mut graph, names_with_ids) = build_graph(&connections);

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
    let connections = Connections::from_str(input).expect("Should parse");

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

    fn count_of_visiting_paths(
        connections: &Connections,
        current_device: DeviceName,
        stopover_a_device: DeviceName,
        stopover_b_device: DeviceName,
        memoisation: &mut HashMap<DeviceName, PathsCount>,
    ) -> PathsCount {
        if let Some(paths_count) = memoisation.get(&current_device) {
            return *paths_count;
        }
        let predecessor_paths = connections
            .0
            .iter()
            .filter(|connection| connection.output.iter().contains(&current_device))
            .map(|connection| &connection.device)
            .collect::<Box<[_]>>();
        if predecessor_paths.is_empty() {
            return PathsCount::zero();
        }
        let mut current_device_paths_count = predecessor_paths
            .iter()
            .map(|predecessor_device| {
                count_of_visiting_paths(
                    connections,
                    (*predecessor_device).clone(),
                    stopover_a_device.clone(),
                    stopover_b_device.clone(),
                    memoisation,
                )
            })
            .fold(PathsCount::zero(), |acc, current| PathsCount {
                from_start: acc.from_start + current.from_start,
                from_start_over_stopover_a: acc.from_start_over_stopover_a
                    + current.from_start_over_stopover_a,
                from_start_over_stopover_b: acc.from_start_over_stopover_b
                    + current.from_start_over_stopover_b,
                from_start_over_both_stopovers: acc.from_start_over_both_stopovers
                    + current.from_start_over_both_stopovers,
            });
        if current_device == stopover_a_device {
            current_device_paths_count.from_start_over_both_stopovers +=
                current_device_paths_count.from_start_over_stopover_b;
            current_device_paths_count.from_start_over_stopover_a +=
                current_device_paths_count.from_start;
        }
        if current_device == stopover_b_device {
            current_device_paths_count.from_start_over_both_stopovers +=
                current_device_paths_count.from_start_over_stopover_a;
            current_device_paths_count.from_start_over_stopover_b +=
                current_device_paths_count.from_start;
        }
        memoisation.insert(current_device, current_device_paths_count);
        current_device_paths_count
    }

    count_of_visiting_paths(
        &connections,
        DeviceName(END.to_string()),
        DeviceName(STOPOVER_A_PART_2.to_string()),
        DeviceName(STOPOVER_B_PART_2.to_string()),
        &mut HashMap::from([(DeviceName(START_PART_2.to_string()), PathsCount::start())]),
    )
    .from_start_over_both_stopovers
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PathsCount {
    from_start: u64,
    from_start_over_stopover_a: u64,
    from_start_over_stopover_b: u64,
    from_start_over_both_stopovers: u64,
}

impl PathsCount {
    fn zero() -> Self {
        Self {
            from_start: 0,
            from_start_over_stopover_a: 0,
            from_start_over_stopover_b: 0,
            from_start_over_both_stopovers: 0,
        }
    }

    fn start() -> Self {
        Self {
            from_start: 1,
            from_start_over_stopover_a: 0,
            from_start_over_stopover_b: 0,
            from_start_over_both_stopovers: 0,
        }
    }
}

#[cfg(feature = "part2")]
fn part2_d(input: &str) -> u64 {
    // Until now 2 hours 53 minutes 15,38 seconds

    fn sub_search_len<'a>(
        connections: &'a Connections,
        from: &DeviceName,
        avoid: &[&DeviceName],
        to: &DeviceName,
        build_reverse: bool,
    ) -> usize {
        //Box<[Box<[()]>]> {
        let (mut graph, names_with_ids) = if build_reverse {
            build_graph_b(connections, to, avoid, from, true)
        } else {
            build_graph_b(connections, from, avoid, to, false)
        };
        println!(
            "BUILT {} nodes {} edges",
            graph.node_count(),
            graph.edge_count()
        );

        let Some(&from_id) = names_with_ids.get(from) else {
            return 0; //Box::new([]);
        };
        let to_id = *names_with_ids.get(to).expect("Should contain to device");

        graph.calculate(from_id, None);
        println!("CALCULATED");
        // TODO println!("graph = {graph:#?}");
        //let all_paths = graph.all_paths(to_id);
        //all_paths.len()
        graph.recursive_all_paths_len(to_id)
        /*println!("PATHS");
        all_paths
            .into_iter()
            .unique()
            .map(|path| {
                path.into_iter()
                    .map(|id| {
                        graph
                            .node(id)
                            .expect("Should contain node")
                            .borrow()
                            .value()
                    })
                    .collect::<Box<[_]>>()
            })
            .collect::<Box<[_]>>()*/
    }

    let connections = Connections::from_str(input).expect("Should parse");
    let start = DeviceName(START_PART_2.to_string());
    let stopover_a = DeviceName(STOPOVER_A_PART_2.to_string());
    let stopover_b = DeviceName(STOPOVER_B_PART_2.to_string());
    let end = DeviceName(END.to_string());

    println!();
    let paths_start_stopover_a = sub_search_len(
        &connections,
        &start,
        &[&stopover_b, &end],
        &stopover_a,
        true,
    );
    println!("CALCULATED START->STOPOVER A = {}", paths_start_stopover_a);
    println!("{paths_start_stopover_a:?}");
    println!();

    let paths_start_stopover_b = sub_search_len(
        &connections,
        &start,
        &[&stopover_a, &end],
        &stopover_b,
        true,
    );
    println!("CALCULATED START->STOPOVER B = {}", paths_start_stopover_b);
    println!("{paths_start_stopover_b:?}");
    println!();

    let paths_stopover_a_b = sub_search_len(
        &connections,
        &stopover_a,
        &[&start, &end],
        &stopover_b,
        true,
    );
    println!("CALCULATED STOPOVER A->STOPOVER B = {}", paths_stopover_a_b);
    println!("{paths_stopover_a_b:?}");
    println!();

    let paths_stopover_b_a = sub_search_len(
        &connections,
        &stopover_b,
        &[&start, &end],
        &stopover_a,
        true,
    );
    println!("CALCULATED STOPOVER B->STOPOVER A = {}", paths_stopover_b_a);
    println!("{paths_stopover_b_a:?}");
    println!();

    let paths_stopover_a_end = sub_search_len(
        &connections,
        &stopover_a,
        &[&start, &stopover_b],
        &end,
        true,
    );
    println!("CALCULATED STOPOVER A->END = {}", paths_stopover_a_end);
    println!("{paths_stopover_a_end:?}");
    println!();

    let paths_stopover_b_end = sub_search_len(
        &connections,
        &stopover_b,
        &[&start, &stopover_a],
        &end,
        false,
    );
    println!("CALCULATED STOPOVER B->END = {}", paths_stopover_b_end);
    println!("{paths_stopover_b_end:?}");
    println!();

    ((paths_start_stopover_a * paths_stopover_a_b * paths_stopover_b_end)
        + (paths_start_stopover_b * paths_stopover_b_a * paths_stopover_a_end)) as u64

    /*
    let stopover_b = *names_with_ids
        .get(&stopover_b)
        .expect("Should contain stopover b device");
    let end = *names_with_ids.get(&end).expect("Should contain end device");*/

    // Calculate paths from:
    // - start -> stopover A
    // - start -> stopover B
    // - stopover A -> stopover B
    // - stopover B -> stopover A
    // - stopover A -> end
    // - stopover B -> end

    /*graph.calculate(start, None);
    println!("Calculate START");
    let paths_start_stopover_a = graph.all_paths(stopover_a);
    println!("paths_start_stopover_a: {}", paths_start_stopover_a.len());
    let paths_start_stopover_b = graph.all_paths(stopover_b);
    println!("paths_start_stopover_b: {}", paths_start_stopover_b.len());

    graph.calculate(stopover_a, None);
    println!("Calculate STOPOVER_A");
    let paths_stopover_a_stopover_b = graph.all_paths(stopover_b);
    println!(
        "paths_stopover_a_stopover_b: {}",
        paths_stopover_a_stopover_b.len()
    );
    let paths_stopover_a_end = graph.all_paths(end);
    println!("paths_stopover_a_end: {}", paths_stopover_a_end.len());

    graph.calculate(stopover_b, None);
    println!("Calculate STOPOVER_B");
    let paths_stopover_b_stopover_a = graph.all_paths(stopover_a);
    println!(
        "paths_stopover_b_stopover_a: {}",
        paths_stopover_b_stopover_a.len()
    );
    let paths_stopover_b_end = graph.all_paths(end);
    println!("paths_stopover_b_end: {}", paths_stopover_b_end.len());*/

    /*let paths = graph
    .all_paths(*end)
    .into_iter()
    .filter(|path| {
        path.iter().any(|id| {
            names_with_ids
                .iter()
                .find(|(_, name_id)| id == *name_id)
                .expect("Should find id")
                .0
                .0
                == "dac"
        }) && path.iter().any(|id| {
            names_with_ids
                .iter()
                .find(|(_, name_id)| id == *name_id)
                .expect("Should find id")
                .0
                .0
                == "fft"
        })
    })
    .unique()
    .collect::<Box<[_]>>();*/
    /*println!(
        "paths: {:#?}",
        paths
            .iter()
            .map(|path| path
                .iter()
                .map(|id| graph
                    .node(*id)
                    .expect("Should get node for id")
                    .borrow()
                    .value()
                    .clone())
                .collect::<Box<[_]>>())
            .collect::<Box<[_]>>()
    );*/
}

fn build_graph(connections: &Connections) -> (Graph<DeviceName>, HashMap<&DeviceName, NodeId>) {
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
    (graph, names_with_ids)
}
fn build_graph_b<'a>(
    connections: &'a Connections,
    from: &'a DeviceName,
    avoid: &[&DeviceName],
    to: &DeviceName,
    reverse: bool,
) -> (Graph<DeviceName>, HashMap<&'a DeviceName, NodeId>) {
    let mut graph = Graph::new();
    let mut names_with_ids = HashMap::from([(from, graph.add_node((*from).clone()))]);

    let mut open = VecDeque::from([from]);
    while let Some(current) = open.pop_front() {
        if current == to {
            continue;
        }
        let current_node_id = *names_with_ids.get(current).expect("Should get Node Id");
        if reverse {
            let found_connections = connections
                .0
                .iter()
                .filter(|connection| connection.output.contains(current))
                .collect::<Box<[_]>>();
            for input in found_connections {
                if avoid.contains(&&input.device) {
                    continue;
                }
                let input_node_id = match names_with_ids.get(&input.device) {
                    None => {
                        let input_node_id = graph.add_node(input.device.clone());
                        names_with_ids.insert(&input.device, input_node_id);
                        open.push_back(&input.device);
                        input_node_id
                    }
                    Some(id) => *id,
                };
                graph
                    .add_edge(input_node_id, current_node_id, 1)
                    .expect("Should add edge to graph");
            }
        } else {
            let current_connection = connections
                .0
                .iter()
                .find(|connection| connection.device == *current)
                .expect("Should find an outgoing connection for device");
            for output in &current_connection.output {
                if avoid.contains(&output) {
                    continue;
                }
                let output_node_id = match names_with_ids.get(output) {
                    None => {
                        let output_node_id = graph.add_node((*output).clone());
                        names_with_ids.insert(output, output_node_id);
                        open.push_back(output);
                        output_node_id
                    }
                    Some(id) => *id,
                };
                graph
                    .add_edge(current_node_id, output_node_id, 1)
                    .expect("Should add edge to graph");
            }
        }
    }

    (graph, names_with_ids)
}

mod graph {
    use itertools::{Either, Itertools};
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashSet};
    use std::iter::once;
    use std::rc::Rc;
    use std::time::{Duration, Instant};

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
                value: value,
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

        pub fn node_count(&self) -> usize {
            self.nodes.len()
        }

        pub fn edge_count(&self) -> usize {
            self.edges.len()
        }

        pub fn recursive_all_paths_len(&self, from: NodeId) -> usize {
            let from_node = self
                .nodes
                .iter()
                .find(|node| node.borrow().id == from)
                .expect("Should find node");
            from_node
                .borrow()
                .distance_from_start
                .iter()
                .map(|(previous_id, _)| {
                    if from == *previous_id {
                        1
                    } else {
                        self.recursive_all_paths_len(*previous_id)
                    }
                })
                .sum()
        }

        fn recursive_all_paths(&self, current_id: NodeId) -> Box<[Box<[NodeId]>]> {
            let current_node = self
                .nodes
                .iter()
                .find(|node| node.borrow().id == current_id)
                .expect("Should find node");
            current_node
                .borrow()
                .distance_from_start
                .iter()
                .flat_map(|(previous_id, _)| {
                    if current_id == *previous_id {
                        Either::Left(once(Box::new([current_id]) as Box<[_]>))
                    } else {
                        Either::Right(
                            self.recursive_all_paths(*previous_id)
                                .into_iter()
                                .map(|path| {
                                    let mut path = path.into_vec();
                                    path.push(current_id);
                                    path.into_boxed_slice()
                                })
                                .unique(),
                        )
                    }
                })
                .collect::<Box<[_]>>()
        }

        pub fn all_paths(&self, to: NodeId) -> Box<[Box<[NodeId]>]> {
            self.recursive_all_paths(to)

            /*let mut paths = vec![vec![to]];
            loop {
                let mut current_paths = Vec::new();
                std::mem::swap(&mut current_paths, &mut paths);

                let mut increased = false;
                for path in current_paths {
                    let first = path.first().expect("Should have first element in path");
                    for (previous, _) in &self
                        .nodes
                        .iter()
                        .find(|node| node.borrow().id == *first)
                        .expect("Should find node for id")
                        .borrow()
                        .distance_from_start
                    {
                        if first == previous {
                            paths.push(path.clone());
                        } else {
                            let mut new_path = path.clone();
                            new_path.insert(0, *previous);
                            increased = true;
                            paths.push(new_path);
                        }
                    }
                }
                if !increased {
                    break;
                }
            }
            paths
                .into_iter()
                .map(|path| path.into_boxed_slice())
                .collect()*/
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

            let mut time = Instant::now();
            while let Some(current) = next.pop() {
                if time.elapsed() > Duration::from_secs(2) {
                    println!("visited={} next={}", visited.len(), next.len());
                    time = Instant::now();
                }
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
                    if !visited.contains(neighbour) && !next.iter().contains(neighbour) {
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
        value: V,
        distance_from_start: Vec<(NodeId, u64)>,
    }

    impl<V> Node<V> {
        pub fn value(&self) -> &V {
            &self.value
        }

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

const START_PART_1: &str = "you";
const START_PART_2: &str = "svr";
const STOPOVER_A_PART_2: &str = "fft";
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
