use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = include_str!("../input/input.day02");
    let part1 = part1(input);
    println!("Solution to part 1: {part1}");
}

fn part1(input: &str) -> u64 {
    // Took 58 minutes 48,25 seconds (excluding breaks of around 60 minutes because of coworkers)
    sum_of_all_invalid_ids(input.parse().expect("Should parse fine"))
}

fn sum_of_all_invalid_ids(ranges: IdRanges) -> u64 {
    ranges.invalid_count().expect("Should calculate")
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct IdRanges(Box<[IdRange]>);

impl IdRanges {
    fn invalid_count(&self) -> Result<u64, InvalidCountIdRangesError> {
        self.0
            .iter()
            .enumerate()
            .map(|(index, range)| {
                range.invalid_count().map_err(|error| {
                    InvalidCountIdRangesError::InvalidCountForRange {
                        index,
                        source: error,
                    }
                })
            })
            .try_fold(0u64, |acc, val| val.map(|val| val + acc))
    }
}

#[derive(thiserror::Error, Debug)]
enum InvalidCountIdRangesError {
    #[error("Failed to get invalid count of range at index '{index}': {source}")]
    InvalidCountForRange {
        index: usize,
        source: InvalidCountIdRangeError,
    },
}

impl FromStr for IdRanges {
    type Err = ParseIdRangesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .split(',')
                .enumerate()
                .map(|(index, range_str)| {
                    range_str
                        .parse::<IdRange>()
                        .map_err(|error| ParseIdRangesError::ParseIdRange {
                            index,
                            source: error,
                        })
                })
                .collect::<Result<Box<[_]>, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseIdRangesError {
    #[error("Failed to parse id range at index '{index}': {source}")]
    ParseIdRange {
        index: usize,
        source: ParseIdRangeError,
    },
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct IdRange {
    from: Id,
    to: Id,
}

impl IdRange {
    fn invalid_count(&self) -> Result<u64, InvalidCountIdRangeError> {
        Ok(self
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(index, current)| {
                current
                    .map(|id| (!id.is_valid()).then_some((index, id)))
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(index, id)| {
                id.0.parse::<u64>()
                    .map_err(|error| InvalidCountIdRangeError::Parse {
                        index,
                        source: error,
                    })
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .sum::<u64>())
    }
}

#[derive(thiserror::Error, Debug)]
enum InvalidCountIdRangeError {
    #[error("Failed to iterate over id range: {0}")]
    Iterate(#[from] IterateIdRangeError),
    #[error("Failed to parse id at index '{index}': {source}")]
    Parse { index: usize, source: ParseIntError },
}

impl IntoIterator for IdRange {
    type Item = <IdRangeIterator as Iterator>::Item;
    type IntoIter = IdRangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        IdRangeIterator {
            next: Some(Ok(self.from)),
            end: self.to,
        }
    }
}

struct IdRangeIterator {
    next: Option<Result<Id, IterateIdRangeError>>,
    end: Id,
}

impl Iterator for IdRangeIterator {
    type Item = Result<Id, IterateIdRangeError>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;
        if let Ok(next) = &next
            && *next != self.end
        {
            self.next = Some(next.next().map_err(|error| IterateIdRangeError::Increase {
                last: next.clone(),
                source: error,
            }));
        }
        Some(next)
    }
}

#[derive(thiserror::Error, Debug)]
enum IterateIdRangeError {
    #[error("Failed to get next id after '{last}': {source}")]
    Increase { last: Id, source: NextIdError },
}

impl FromStr for IdRange {
    type Err = ParseIdRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from_str, to_str) = s
            .split_once('-')
            .ok_or(ParseIdRangeError::MissingDelimiter)?;
        Ok(Self {
            from: from_str
                .parse()
                .map_err(|error| ParseIdRangeError::ParseFrom {
                    value: from_str.to_string(),
                    source: error,
                })?,
            to: to_str.parse().map_err(|error| ParseIdRangeError::ParseTo {
                value: to_str.to_string(),
                source: error,
            })?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseIdRangeError {
    #[error("Missing '-' delimiter")]
    MissingDelimiter,
    #[error("Failed to parse '{value}' as from part: {source}")]
    ParseFrom { value: String, source: ParseIdError },
    #[error("Failed to parse '{value}' as to part: {source}")]
    ParseTo { value: String, source: ParseIdError },
}

#[derive(derive_more::Display, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Id(String);

impl Id {
    fn is_valid(&self) -> bool {
        if self.0.starts_with('0') {
            false
        } else if self.0.len() > 1
            && let Some((left, right)) = self.0.split_at_checked(self.0.len() / 2)
            && left == right
        {
            false
        } else {
            true
        }
    }

    fn next(&self) -> Result<Self, NextIdError> {
        Ok(Self(
            self.0
                .parse::<u64>()?
                .checked_add(1)
                .ok_or(NextIdError::Overflow)?
                .to_string(),
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum NextIdError {
    #[error("Failed to parse internal value: {0}")]
    Parse(#[from] ParseIntError),
    #[error("Failed to get next id as overflow has happened")]
    Overflow,
}

impl FromStr for Id {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(char::is_numeric) {
            Ok(Self(s.to_string()))
        } else {
            Err(ParseIdError::NonNumeric)
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to parse id: {0}")]
enum ParseIdError {
    #[error("String contains non-numeric character")]
    NonNumeric,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 1227775554);
    }
}
