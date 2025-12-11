use std::fmt::{Display, Formatter};
use std::iter::once;
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
}

const INPUT: &str = include_str!("../input/input.day07");

fn part1(input: &str) -> u64 {
    // Took 34 minutes 35,81 seconds
    TachyonManifold::from_str(input)
        .expect("Should parse")
        .run_tachyon_beam()
        .split_count()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct TachyonManifold {
    area: Vec<Vec<Field>>,
    split_count: u64,
}

impl TachyonManifold {
    fn run_tachyon_beam(&mut self) -> &mut Self {
        for row_index in 0..self.area.len().saturating_sub(1) {
            for column_index in 0..self.area[row_index].len() {
                match self.area[row_index][column_index] {
                    Field::Start | Field::Beam => match self.area[row_index + 1][column_index] {
                        Field::Start => unreachable!("There will be no start besides first line"),
                        Field::EmptySpace => self.area[row_index + 1][column_index] = Field::Beam,
                        Field::Splitter => {
                            self.split_count += 1;
                            if column_index > 0 {
                                match self.area[row_index + 1][column_index - 1] {
                                    Field::Start => {
                                        unreachable!("There will be no start besides first line")
                                    }
                                    Field::EmptySpace => {
                                        self.area[row_index + 1][column_index - 1] = Field::Beam
                                    }
                                    Field::Splitter => unimplemented!("Do not know what to do now"),
                                    Field::Beam => (),
                                }
                            }
                            if column_index < self.area[row_index].len().saturating_sub(1) {
                                match self.area[row_index + 1][column_index + 1] {
                                    Field::Start => {
                                        unreachable!("There will be no start besides first line")
                                    }
                                    Field::EmptySpace => {
                                        self.area[row_index + 1][column_index + 1] = Field::Beam
                                    }
                                    Field::Splitter => unimplemented!("Do not know what to do now"),
                                    Field::Beam => (),
                                }
                            }
                        }
                        Field::Beam => (),
                    },
                    Field::EmptySpace | Field::Splitter => (),
                }
            }
        }
        self
    }

    fn split_count(&self) -> u64 {
        self.split_count
    }
}

impl FromStr for TachyonManifold {
    type Err = ParseTachyonManifoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines();
        let Some(first_line) = lines_iter.next() else {
            return Ok(Self {
                area: Vec::new(),
                split_count: 0,
            });
        };
        let expected_columns = first_line.len();
        Ok(Self {
            area: once(first_line)
                .chain(lines_iter)
                .map(|line| {
                    let line = line
                        .chars()
                        .map(|field| match field {
                            '.' => Ok(Field::EmptySpace),
                            '^' => Ok(Field::Splitter),
                            'S' => Ok(Field::Start),
                            _ => Err(ParseTachyonManifoldError::UnknownField { field }),
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    if line.len() != expected_columns {
                        Err(ParseTachyonManifoldError::UnexpectedColumnCount { count: line.len() })
                    } else {
                        Ok(line)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
            split_count: 0,
        })
    }
}

impl Display for TachyonManifold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row_index in 0..self.area.len() {
            for column_index in 0..self.area[row_index].len() {
                match self.area[row_index][column_index] {
                    Field::Start => write!(f, "S")?,
                    Field::EmptySpace => write!(f, ".")?,
                    Field::Splitter => write!(f, "^")?,
                    Field::Beam => write!(f, "|")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseTachyonManifoldError {
    #[error("Unknown field character '{field}'")]
    UnknownField { field: char },
    #[error("Line has an unexpected column count of '{count}'")]
    UnexpectedColumnCount { count: usize },
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Field {
    Start,
    EmptySpace,
    Splitter,
    Beam,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 21);
    }
}
