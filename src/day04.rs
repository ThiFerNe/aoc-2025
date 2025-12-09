use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
}

const INPUT: &str = include_str!("../input/input.day04");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 35 minutes 8,96 seconds
    count_of_paper_rolls_accessible_by_a_forklift(input.parse().expect("Should parse department"))
}

fn count_of_paper_rolls_accessible_by_a_forklift(department: PrintingDepartment) -> u64 {
    let mut count = 0;
    for y in 0..department.grid.len() {
        for x in 0..department.grid[y].len() {
            if department.is_paper_roll(y, x) && department.count_neighbours(y, x) < 4 {
                count += 1;
            }
        }
    }
    count
}

struct PrintingDepartment {
    grid: Vec<Vec<Option<PaperRoll>>>,
}

impl PrintingDepartment {
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
                    .and_then(|row| row.get(check_column_index).map(|cell| cell.is_some()))
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
            .and_then(|row| row.get(column_index).map(|cell| cell.is_some()))
            .unwrap_or(false)
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
                        '.' => Ok(None),
                        '@' => Ok(Some(PaperRoll)),
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

struct PaperRoll;

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
}
