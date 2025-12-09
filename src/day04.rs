use std::fmt::{Display, Formatter};
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day04");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 35 minutes 8,96 seconds
    count_of_paper_rolls_accessible_by_a_forklift(input.parse().expect("Should parse department"))
}

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 20 minutes 23,20 seconds
    count_of_paper_rolls_removable_repeatedly(input.parse().expect("Should parse department"))
}

#[cfg(feature = "part1")]
fn count_of_paper_rolls_accessible_by_a_forklift(mut department: PrintingDepartment) -> u64 {
    department.mark_removable()
}

#[cfg(feature = "part2")]
fn count_of_paper_rolls_removable_repeatedly(mut department: PrintingDepartment) -> u64 {
    let mut count = 0;
    loop {
        department.mark_removable();
        let new_count = department.remove_removable();
        if new_count == 0 {
            break;
        }
        count += new_count;
    }
    count
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct PrintingDepartment {
    grid: Vec<Vec<MaybePaperRoll>>,
}

impl PrintingDepartment {
    fn mark_removable(&mut self) -> u64 {
        let mut count = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.is_paper_roll(y, x) {
                    if self.count_neighbours(y, x) < 4 {
                        self.grid[y][x] = MaybePaperRoll::Removable;
                        count += 1;
                    } else {
                        self.grid[y][x] = MaybePaperRoll::Irremovable;
                    }
                }
            }
        }
        count
    }

    #[cfg(feature = "part2")]
    fn remove_removable(&mut self) -> u64 {
        let mut count = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                self.grid[y][x] = match self.grid[y][x] {
                    MaybePaperRoll::MovementUnchecked | MaybePaperRoll::Irremovable => {
                        MaybePaperRoll::MovementUnchecked
                    }
                    MaybePaperRoll::Removable => {
                        count += 1;
                        MaybePaperRoll::None
                    }
                    MaybePaperRoll::None => MaybePaperRoll::None,
                };
            }
        }
        count
    }

    fn count_neighbours(&self, row_index: usize, column_index: usize) -> u64 {
        let mut count = 0;
        for check_row_relative_index in -1isize..=1 {
            let Some(check_row_index) = row_index.checked_add_signed(check_row_relative_index)
            else {
                continue;
            };
            for check_column_relative_index in -1isize..=1 {
                if check_row_relative_index == 0 && check_column_relative_index == 0 {
                    continue;
                }
                let Some(check_column_index) =
                    column_index.checked_add_signed(check_column_relative_index)
                else {
                    continue;
                };
                let is_neighbour = self
                    .grid
                    .get(check_row_index)
                    .and_then(|row| {
                        row.get(check_column_index)
                            .map(MaybePaperRoll::is_paper_roll)
                    })
                    .unwrap_or(false);
                if is_neighbour {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_paper_roll(&self, row_index: usize, column_index: usize) -> bool {
        self.grid
            .get(row_index)
            .and_then(|row| row.get(column_index).map(MaybePaperRoll::is_paper_roll))
            .unwrap_or(false)
    }
}

impl Display for PrintingDepartment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                match self.grid[y][x] {
                    MaybePaperRoll::MovementUnchecked => write!(f, "@")?,
                    MaybePaperRoll::Removable => write!(f, "X")?,
                    MaybePaperRoll::Irremovable => write!(f, "O")?,
                    MaybePaperRoll::None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for PrintingDepartment {
    type Err = ParsePrintingDepartmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = s.lines();
        let Some(first_row) = rows.next() else {
            return Ok(Self { grid: Vec::new() });
        };
        let expected_columns = first_row.len();
        std::iter::once(first_row)
            .chain(rows)
            .enumerate()
            .map(move |(row_index, row)| {
                row.chars()
                    .enumerate()
                    .map(move |(column_index, cell)| match cell {
                        '.' => Ok(MaybePaperRoll::None),
                        '@' => Ok(MaybePaperRoll::MovementUnchecked),
                        _ => Err(ParsePrintingDepartmentError::Unknown {
                            value: cell,
                            row: row_index,
                            column: column_index,
                        }),
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .and_then(|row| {
                        if row.len() == expected_columns {
                            Ok(row)
                        } else {
                            Err(ParsePrintingDepartmentError::InvalidColumnLength {
                                is: row.len(),
                                expected: expected_columns,
                                row: row_index,
                            })
                        }
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|grid| PrintingDepartment { grid })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParsePrintingDepartmentError {
    #[error("Unknown element in row {row} and column {column}: '{value}'")]
    Unknown {
        value: char,
        row: usize,
        column: usize,
    },
    #[error("Row {row} has a length of {is} but expected was {expected}")]
    InvalidColumnLength {
        is: usize,
        expected: usize,
        row: usize,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum MaybePaperRoll {
    MovementUnchecked,
    Removable,
    Irremovable,
    None,
}

impl MaybePaperRoll {
    fn is_paper_roll(&self) -> bool {
        matches!(
            self,
            Self::MovementUnchecked | Self::Removable | Self::Irremovable
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 13);
    }

    #[test]
    fn test_part2() {
        // Arrange
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        // Act
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 43);
    }
}
