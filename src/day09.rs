use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
}

const INPUT: &str = include_str!("../input/input.day09");

fn part1(input: &str) -> u64 {
    // Took 11 minutes 24,12 seconds
    let list = RedTilesList::from_str(input).expect("Should parse");
    let (_, _, area) = list.find_biggest_rectangle().expect("Should not be empty");
    area
}

struct RedTilesList(Box<[RedTileLocation]>);

impl RedTilesList {
    fn find_biggest_rectangle(&self) -> Option<(&RedTileLocation, &RedTileLocation, u64)> {
        (0..self.0.len())
            .flat_map(|first_index| {
                (0..self.0.len()).map(move |second_index| {
                    (
                        &self.0[first_index],
                        &self.0[second_index],
                        self.0[first_index].area_with(&self.0[second_index]),
                    )
                })
            })
            .max_by_key(|(_, _, area)| *area)
    }
}

impl FromStr for RedTilesList {
    type Err = ParseRedTilesListError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseRedTilesListError {
    #[error("Failed to parse red tile location: {0}")]
    ParseRedTileLocation(#[from] ParseRedTileLocationError),
}

struct RedTileLocation {
    x: u64,
    y: u64,
}

impl RedTileLocation {
    fn area_with(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl FromStr for RedTileLocation {
    type Err = ParseRedTileLocationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s
            .split_once(',')
            .ok_or(ParseRedTileLocationError::MissingDelimiter)?;
        Ok(Self {
            x: x_str.parse()?,
            y: y_str.parse()?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseRedTileLocationError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Failed to parse coordinate part: {0}")]
    ParseCoordinate(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 50);
    }
}
