use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day01");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 33 minutes 2,24 seconds (excluding breaks of around 15 minutes because of cats)
    number_of_times_dial_pointing_at_0_after_rotations(
        &mut Dial::new(50, 99).expect("Should be correct values"),
        &input.parse().expect("Should parse fine"),
    )
}

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 11 minutes 7,26 seconds (again, excluding breaks of around 15 minutes because of cat)
    number_of_time_dial_pointing_at_0_at_any_time(
        &mut Dial::new(50, 99).expect("Should be correct values"),
        &input.parse().expect("Should parse fine"),
    )
}

#[cfg(feature = "part1")]
fn number_of_times_dial_pointing_at_0_after_rotations(
    dial: &mut Dial,
    rotations: &Rotations,
) -> u64 {
    let mut count = 0;
    for rotation in &rotations.0 {
        dial.apply(rotation);
        if dial.pointing_at == 0 {
            count += 1;
        }
    }
    count
}

#[cfg(feature = "part2")]
fn number_of_time_dial_pointing_at_0_at_any_time(dial: &mut Dial, rotations: &Rotations) -> u64 {
    let mut count = 0;
    for rotation in &rotations.0 {
        let ran_over_zero = dial.apply(rotation);
        count += ran_over_zero.0;
        if dial.pointing_at == 0 {
            count += 1;
        }
    }
    count
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Dial {
    pointing_at: u64,
    maximum_value: u64,
}

impl Dial {
    fn new(pointing_at: u64, maximum_value: u64) -> Result<Dial, NewDialError> {
        if pointing_at > maximum_value {
            Err(NewDialError::Invalid)
        } else {
            Ok(Self {
                pointing_at,
                maximum_value,
            })
        }
    }
    fn apply(&mut self, rotation: &Rotation) -> RanOverZero {
        let remaining_non_complete_rotations = rotation.distance % (self.maximum_value + 1);
        let full_rotations = rotation.distance / (self.maximum_value + 1);
        match rotation.direction {
            Direction::Left => {
                let (new_pointing_at, ran_over_zero) =
                    match remaining_non_complete_rotations.cmp(&self.pointing_at) {
                        Ordering::Less => (self.pointing_at - remaining_non_complete_rotations, 0),
                        Ordering::Equal => (0, 0),
                        Ordering::Greater => (
                            (self.maximum_value + 1)
                                - (remaining_non_complete_rotations - self.pointing_at),
                            if self.pointing_at != 0 { 1 } else { 0 },
                        ),
                    };
                self.pointing_at = new_pointing_at;
                RanOverZero(full_rotations + ran_over_zero)
            }
            Direction::Right => {
                let new_pointing_at = self.pointing_at + remaining_non_complete_rotations;
                let (new_pointing_at, ran_over_zero) = if new_pointing_at > (self.maximum_value + 1)
                {
                    (new_pointing_at - (self.maximum_value + 1), 1)
                } else if new_pointing_at == (self.maximum_value + 1) {
                    (new_pointing_at - (self.maximum_value + 1), 0)
                } else {
                    (new_pointing_at, 0)
                };
                self.pointing_at = new_pointing_at;
                RanOverZero(full_rotations + ran_over_zero)
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum NewDialError {
    #[error("Failed to create Dial as ")]
    Invalid,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct RanOverZero(u64);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Rotations(Box<[Rotation]>);

impl FromStr for Rotations {
    type Err = ParseRotationsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .enumerate()
                .map(|(index, line)| {
                    line.parse()
                        .map_err(|error| ParseRotationsError::ParseRotationError {
                            index,
                            source: error,
                        })
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseRotationsError {
    #[error("Failed to parse rotation at line {index}: {source}")]
    ParseRotationError {
        index: usize,
        source: ParseRotationError,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Rotation {
    direction: Direction,
    distance: u64,
}

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at_checked(1) {
            Some((direction_str, distance_str)) => Ok(Self {
                direction: direction_str.parse()?,
                distance: distance_str.parse()?,
            }),
            None => Err(ParseRotationError::Empty),
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseRotationError {
    #[error("Failed to parse rotation as it is empty")]
    Empty,
    #[error("Failed to parse direction: {0}")]
    ParseDirection(#[from] ParseDirectionError),
    #[error("Failed to parse distance: {0}")]
    ParseDistance(#[from] ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(ParseDirectionError::Unknown),
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseDirectionError {
    #[error("Failed to parse unknown direction key")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 3)
    }

    #[test]
    fn test_part_2() {
        // Arrange
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        // Act
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 6)
    }
}
