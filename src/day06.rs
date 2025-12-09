use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day06");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 24 minutes 46,45 seconds
    Worksheet::parse(input, ParseKind::TopToBottom)
        .expect("Should parse")
        .grand_total()
}

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 54 minutes 40,35 seconds
    Worksheet::parse(input, ParseKind::RightToLeft)
        .expect("Should parse")
        .grand_total()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Worksheet(Box<[Problem]>);

impl Worksheet {
    fn grand_total(&self) -> u64 {
        self.0
            .iter()
            .map(Problem::solve)
            .reduce(|left, right| left.checked_add(right).expect("Should not overflow"))
            .expect("Should not be empty")
    }

    fn parse(s: &str, kind: ParseKind) -> Result<Self, ParseWorksheetError> {
        let mut lines_iter = s.lines().filter(|line| !line.is_empty());

        let problem_kind_line = lines_iter.next_back().ok_or(ParseWorksheetError::Empty)?;
        let mut problem_kinds: Vec<ProblemKind> = Vec::new();
        let mut column_widths = Vec::new();

        let mut current_index = 0;
        while let Some(next_kind_index) =
            problem_kind_line
                .get((current_index + 1)..)
                .and_then(|slice| {
                    slice
                        .find(|c: char| !c.is_whitespace())
                        .map(|index| index + current_index + 1)
                })
        {
            problem_kinds.push(
                problem_kind_line[current_index..next_kind_index]
                    .trim()
                    .parse()?,
            );
            column_widths.push(next_kind_index - current_index - 1);
            current_index = next_kind_index;
        }
        problem_kinds.push(problem_kind_line[current_index..].trim().parse()?);
        column_widths.push(problem_kind_line.len() - current_index);

        let mut input_cells: Vec<Vec<&str>> = Vec::with_capacity(problem_kinds.len());
        for line in lines_iter {
            let mut line_offset = 0;
            for (column_index, column_width) in column_widths.iter().enumerate() {
                let cell = &line[line_offset..][..*column_width];
                match input_cells.get_mut(column_index) {
                    Some(problem) => problem.push(cell),
                    None => input_cells.push(vec![cell]),
                }
                line_offset += column_width + 1;
            }
        }

        Ok(Self(
            problem_kinds
                .into_iter()
                .zip(input_cells.into_iter())
                .map(|(problem_kind, cells)| Ok::<_, ParseWorksheetError>(Problem {
                    numbers: match kind {
                        ParseKind::TopToBottom => cells
                            .into_iter()
                            .map(|cell| cell.trim().parse()
                                .map_err(ParseWorksheetError::ParseNumber))
                            .collect::<Result<_, _>>()?,
                        ParseKind::RightToLeft => {
                            (0..cells.first().expect("Should have at least one cell").len())
                                .rev()
                                .map(|column_index| {
                                    cells
                                        .iter()
                                        .map(|cell| cell.chars().nth(column_index).expect("Should have at least same amount of chars as first cell"))
                                        .collect::<String>()
                                        .trim()
                                        .parse()
                                        .map_err(ParseWorksheetError::ParseNumber)
                                })
                                .collect::<Result<_, _>>()?
                        }
                    },
                    kind: problem_kind,
                }))
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum ParseKind {
    #[default]
    TopToBottom,
    RightToLeft,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseWorksheetError {
    #[error("Worksheet is empty")]
    Empty,
    #[error("Failed to parse problem number: {0}")]
    ParseNumber(#[from] ParseIntError),
    #[error("Failed to parse problem kind: {0}")]
    ParseKind(#[from] ParseProblemKindError),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Problem {
    numbers: Vec<u64>,
    kind: ProblemKind,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.kind {
            ProblemKind::Add => self
                .numbers
                .iter()
                .copied()
                .reduce(|left, right| left.checked_add(right).expect("Should not overflow"))
                .expect("Should not be empty"),
            ProblemKind::Multiply => self
                .numbers
                .iter()
                .copied()
                .reduce(|left, right| left.checked_mul(right).expect("Should not overflow"))
                .expect("Should not be empty"),
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
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 4277556);
    }

    #[test]
    fn test_part2() {
        // Arrange
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        // Act
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 3263827);
    }
}
