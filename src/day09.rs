use std::num::ParseIntError;
use std::str::FromStr;

#[cfg(feature = "part2")]
use itertools::Itertools;

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day09");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 11 minutes 24,12 seconds
    let list = RedTilesList::from_str(input).expect("Should parse");
    let (_, _, area) = list.find_biggest_rectangle().expect("Should not be empty");
    area
}

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 2 hours 28 minutes 19,48 seconds
    let list = RedTilesList::from_str(input).expect("Should parse");
    let (_, _, area) = list
        .find_biggest_rectangle_in_bounding_box()
        .expect("Should not be empty");
    area
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct RedTilesList(Box<[RedTileLocation]>);

impl RedTilesList {
    #[cfg(feature = "part1")]
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

    #[cfg(feature = "part2")]
    fn find_biggest_rectangle_in_bounding_box(
        &self,
    ) -> Option<(&RedTileLocation, &RedTileLocation, u64)> {
        fn line_in_rectangle(
            line: (&RedTileLocation, &RedTileLocation),
            rectangle: (&RedTileLocation, &RedTileLocation),
        ) -> bool {
            line.0.x.min(line.1.x) < rectangle.0.x.max(rectangle.1.x)
                && line.0.x.max(line.1.x) > rectangle.0.x.min(rectangle.1.x)
                && line.0.y.min(line.1.y) < rectangle.0.y.max(rectangle.1.y)
                && line.0.y.max(rectangle.1.y) > rectangle.0.y.min(rectangle.1.y)
        }

        (0..self.0.len())
            .cartesian_product(0..self.0.len())
            .filter(|(first_index, second_index)| {
                let first = &self.0[*first_index];
                let second = &self.0[*second_index];

                for index in 0..self.0.len() {
                    let point_a = if index == 0 {
                        &self.0[self.0.len() - 1]
                    } else {
                        &self.0[index - 1]
                    };
                    let point_b = &self.0[index];
                    let line = (point_a, point_b);

                    if line_in_rectangle(line, (first, second)) {
                        return false;
                    }
                }

                true
            })
            .map(|(first_index, second_index)| {
                (
                    &self.0[first_index],
                    &self.0[second_index],
                    self.0[first_index].area_with(&self.0[second_index]),
                )
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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

    #[test]
    fn test_part2() {
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
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 24);
    }
}
