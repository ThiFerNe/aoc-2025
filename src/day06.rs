use std::iter::once;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
}

const INPUT: &str = include_str!("../input/input.day06");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 24 minutes 46,45 seconds
    Worksheet::from_str(input)
        .expect("Should parse")
        .grand_total()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Worksheet(Box<[Problem]>);

impl Worksheet {
    fn grand_total(&self) -> u64 {
        self.0.iter().map(Problem::solve).sum()
    }
}

impl FromStr for Worksheet {
    type Err = ParseWorksheetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut problem_numbers: Vec<Vec<u64>> = Vec::new();
        for (line_index, line) in s.lines().enumerate() {
            let mut cells = line.split_whitespace().filter(|s| !s.is_empty());
            let first_cell = cells.next().expect("Should not have empty line");
            if first_cell
                .chars()
                .next()
                .expect("Should not be empty")
                .is_numeric()
            {
                for (column_index, cell) in once(first_cell).chain(cells).enumerate() {
                    let cell = cell
                        .parse()
                        .map_err(|error| ParseWorksheetError::ParseNumber {
                            source: error,
                            line_index,
                            column_index,
                            value: cell.to_string(),
                        })?;
                    match problem_numbers.get_mut(column_index) {
                        Some(problem) => problem.push(cell),
                        None => problem_numbers.push(vec![cell]),
                    }
                }
            } else {
                return Ok(Self(
                    once(first_cell)
                        .chain(cells)
                        .map(|cell| cell.parse().map_err(ParseWorksheetError::ParseKind))
                        .zip(problem_numbers)
                        .map(|(problem_kind_result, problem_numbers)| {
                            problem_kind_result.map(|problem_kind| Problem {
                                numbers: problem_numbers,
                                kind: problem_kind,
                            })
                        })
                        .collect::<Result<_, _>>()?,
                ));
            }
        }
        Err(ParseWorksheetError::MissingProblemKinds)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseWorksheetError {
    #[error(
        "Failed to parse problem number at line '{line_index}' and column '{column_index}' with value '{value}': {source}"
    )]
    ParseNumber {
        source: ParseIntError,
        line_index: usize,
        column_index: usize,
        value: String,
    },
    #[error("Failed to parse problem kind: {0}")]
    ParseKind(#[from] ParseProblemKindError),
    #[error("No problem kinds have been found")]
    MissingProblemKinds,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Problem {
    numbers: Vec<u64>,
    kind: ProblemKind,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.kind {
            ProblemKind::Add => self.numbers.iter().sum(),
            ProblemKind::Multiply => self.numbers.iter().product(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum ProblemKind {
    Add,
    Multiply,
}

impl FromStr for ProblemKind {
    type Err = ParseProblemKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(ProblemKind::Add),
            "*" => Ok(ProblemKind::Multiply),
            _ => Err(ParseProblemKindError::Unknown),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseProblemKindError {
    #[error("Unknown problem kind")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 4277556);
    }
}
