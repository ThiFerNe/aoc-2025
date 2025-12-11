use std::fmt::{Display, Formatter};
use std::iter::once;

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day07");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 34 minutes 35,81 seconds
    TachyonManifold::parse(input, TachyonManifoldKind::Classical)
        .expect("Should parse")
        .run_tachyon_beam()
        .split_count()
}

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 19 minutes 4,20 seconds
    TachyonManifold::parse(input, TachyonManifoldKind::Quantum)
        .expect("Should parse")
        .run_tachyon_beam()
        .timelines_count()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct TachyonManifold {
    kind: TachyonManifoldKind,
    area: Vec<Vec<Field>>,
    split_count: u64,
}

impl TachyonManifold {
    fn parse(s: &str, kind: TachyonManifoldKind) -> Result<Self, ParseTachyonManifoldError> {
        let mut lines_iter = s.lines();
        let Some(first_line) = lines_iter.next() else {
            return Ok(Self {
                kind,
                area: Vec::new(),
                split_count: 0,
            });
        };
        let expected_columns = first_line.len();
        Ok(Self {
            kind,
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
    fn run_tachyon_beam(&mut self) -> &mut Self {
        for row_index in 0..self.area.len().saturating_sub(1) {
            for column_index in 0..self.area[row_index].len() {
                let beam_to_be_propagated_count = match self.area[row_index][column_index] {
                    Field::EmptySpace | Field::Splitter => 0,
                    Field::Start => 1,
                    Field::Beam { count_in_timelines } => count_in_timelines,
                };
                if beam_to_be_propagated_count > 0 {
                    match self.area[row_index + 1][column_index] {
                        Field::Start => unreachable!("There will be no start besides first line"),
                        Field::EmptySpace => {
                            self.area[row_index + 1][column_index] = Field::Beam {
                                count_in_timelines: beam_to_be_propagated_count,
                            }
                        }
                        Field::Splitter => {
                            self.split_count += 1;
                            if column_index > 0 {
                                match self.area[row_index + 1][column_index - 1] {
                                    Field::Start => {
                                        unreachable!("There will be no start besides first line")
                                    }
                                    Field::EmptySpace => {
                                        self.area[row_index + 1][column_index - 1] = Field::Beam {
                                            count_in_timelines: beam_to_be_propagated_count,
                                        }
                                    }
                                    Field::Splitter => unimplemented!("Do not know what to do now"),
                                    Field::Beam { count_in_timelines } => {
                                        self.area[row_index + 1][column_index - 1] = Field::Beam {
                                            count_in_timelines: count_in_timelines
                                                + beam_to_be_propagated_count,
                                        }
                                    }
                                }
                            }
                            if column_index < self.area[row_index].len().saturating_sub(1) {
                                match self.area[row_index + 1][column_index + 1] {
                                    Field::Start => {
                                        unreachable!("There will be no start besides first line")
                                    }
                                    Field::EmptySpace => {
                                        self.area[row_index + 1][column_index + 1] = Field::Beam {
                                            count_in_timelines: beam_to_be_propagated_count,
                                        }
                                    }
                                    Field::Splitter => unimplemented!("Do not know what to do now"),
                                    Field::Beam { count_in_timelines } => {
                                        self.area[row_index + 1][column_index + 1] = Field::Beam {
                                            count_in_timelines: count_in_timelines
                                                + beam_to_be_propagated_count,
                                        }
                                    }
                                }
                            }
                        }
                        Field::Beam { count_in_timelines } => {
                            self.area[row_index + 1][column_index] = Field::Beam {
                                count_in_timelines: count_in_timelines
                                    + beam_to_be_propagated_count,
                            }
                        }
                    }
                }
            }
        }
        self
    }

    fn split_count(&self) -> u64 {
        self.split_count
    }

    fn timelines_count(&self) -> u64 {
        let Some(last_row) = self.area.last() else {
            return 0;
        };
        last_row
            .iter()
            .map(|row| match row {
                Field::Start => 0,
                Field::EmptySpace => 0,
                Field::Splitter => 0,
                Field::Beam { count_in_timelines } => *count_in_timelines,
            })
            .sum()
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
                    Field::Beam { count_in_timelines } => match count_in_timelines {
                        0 => unreachable!(),
                        1 => write!(f, "|")?,
                        c if (2..=9).contains(&c) => write!(f, "{}", c)?,
                        _ => write!(f, "+")?,
                    },
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum TachyonManifoldKind {
    Classical,
    Quantum,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Field {
    Start,
    EmptySpace,
    Splitter,
    Beam { count_in_timelines: u64 },
}

#[derive(thiserror::Error, Debug)]
enum ParseTachyonManifoldError {
    #[error("Unknown field character '{field}'")]
    UnknownField { field: char },
    #[error("Line has an unexpected column count of '{count}'")]
    UnexpectedColumnCount { count: usize },
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

    #[test]
    fn test_part2() {
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
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 40);
    }
}
