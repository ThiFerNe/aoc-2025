use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
}

const INPUT: &str = include_str!("../input/input.day08");

fn part1(input: &str) -> u64 {
    // Took 1 hour 3 minutes 42,82 seconds
    product_of_size_of_largets_circuits(Playground::from_str(input).expect("Should parse"), 1000)
}

fn product_of_size_of_largets_circuits(playground: Playground, connect_count: usize) -> u64 {
    let mut connected_circuits = playground.connect_closest(connect_count);
    connected_circuits.sort_by_key(|circuit| circuit.0.len());
    connected_circuits
        .into_iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.0.len() as u64)
        .product()
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Playground(Box<[JunctionBox]>);

impl Playground {
    fn connect_closest(&self, count: usize) -> Box<[Circuit]> {
        let mut pair_distances: HashMap<JunctionBoxPair, JunctionBoxPairDistance> = HashMap::new();
        for first_index in 0..self.0.len() {
            for second_index in 0..self.0.len() {
                if first_index == second_index {
                    continue;
                }
                let key = JunctionBoxPair {
                    first_index: first_index.min(second_index),
                    second_index: first_index.max(second_index),
                };
                pair_distances.entry(key).or_insert_with(|| {
                    JunctionBoxPairDistance(self.0[first_index].distance_to(&self.0[second_index]))
                });
            }
        }
        let mut circuits = (0..self.0.len())
            .map(|index| vec![index])
            .collect::<Vec<_>>();
        let relevant_pairs = pair_distances
            .into_iter()
            .sorted_by_key(|a| a.1)
            .take(count);
        for (pair, _) in relevant_pairs {
            let first_circuit_index = circuits
                .iter()
                .position(|circuit| circuit.contains(&pair.first_index))
                .expect("Should find junction box in indices");
            let second_circuit_index = circuits
                .iter()
                .position(|circuit| circuit.contains(&pair.second_index))
                .expect("Should find junction box in indices");
            if first_circuit_index != second_circuit_index {
                let new_circuit = circuits[first_circuit_index]
                    .iter()
                    .copied()
                    .chain(circuits[second_circuit_index].iter().copied())
                    .unique()
                    .collect::<Vec<_>>();
                circuits.retain(|circuit| {
                    !circuit.contains(&pair.first_index) && !circuit.contains(&pair.second_index)
                });
                circuits.push(new_circuit);
            }
        }
        circuits
            .into_iter()
            .map(|circuit| Circuit(circuit.into_iter().map(|index| self.0[index]).collect()))
            .collect()
    }
}

impl FromStr for Playground {
    type Err = ParsePlaygroundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParsePlaygroundError {
    #[error("Failed to parse junction box: {0}")]
    ParseJunctionBox(#[from] ParseJunctionBoxError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> u64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
            as u64
    }
}

impl FromStr for JunctionBox {
    type Err = ParseJunctionBoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y, z]: [_; 3] = s
            .split(',')
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ParseJunctionBoxError::CoordinateCountMismatch)?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseJunctionBoxError {
    #[error("Count of coordinates does not match 3")]
    CoordinateCountMismatch,
    #[error("Failed to parse coordinate: {0}")]
    ParseCoordinate(#[from] ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct JunctionBoxPair {
    first_index: usize,
    second_index: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct JunctionBoxPairDistance(u64);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Circuit(Box<[JunctionBox]>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        // Act
        let part1 = product_of_size_of_largets_circuits(
            Playground::from_str(input).expect("Should parse"),
            10,
        );

        // Assert
        assert_eq!(part1, 40);
    }
}
