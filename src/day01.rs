use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    part_1();
}

fn part_1() {
    // Took 33 minutes 2,24 seconds (including breaks of around 15 minutes because of cats)
    let count = number_of_times_dial_pointing_at_0_after_rotations(
        &mut Dial::new(50, 99).expect("Should be correct values"),
        &include_str!("../input/input.day01")
            .parse()
            .expect("Should parse fine"),
    );
    println!("The answer to part 1 is: {count}");
}

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
    fn apply(&mut self, rotation: &Rotation) {
        let remaining_non_complete_rotations = rotation.distance % (self.maximum_value + 1);
        match rotation.direction {
            Direction::Left => {
                self.pointing_at = match remaining_non_complete_rotations.cmp(&self.pointing_at) {
                    Ordering::Less => self.pointing_at - remaining_non_complete_rotations,
                    Ordering::Equal => 0,
                    Ordering::Greater => {
                        (self.maximum_value + 1)
                            - (remaining_non_complete_rotations - self.pointing_at)
                    }
                };
            }
            Direction::Right => {
                self.pointing_at = (self.pointing_at + remaining_non_complete_rotations)
                    % (self.maximum_value + 1);
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum NewDialError {
    #[error("Failed to create Dial as ")]
    Invalid,
}

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
        let mut dial = Dial::new(50, 99).expect("Should be correct values");

        // Act
        let rotations: Rotations = input.parse().expect("Should parse");
        let count = number_of_times_dial_pointing_at_0_after_rotations(&mut dial, &rotations);

        // Assert
        assert_eq!(count, 3)
    }
}
