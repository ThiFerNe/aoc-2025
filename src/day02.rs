use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input/input.day02");
    let part1 = part1(input);
    println!("Solution to part 1: {part1}");
    let part2 = part2(input);
    println!("Solution to part 1: {part2}");
}

fn part1(input: &str) -> u64 {
    // Took 58 minutes 48,25 seconds (excluding breaks of around 60 minutes because of coworkers)
    sum_of_all_invalid_ids(input.parse().expect("Should parse fine"), |id| {
        if id.0.starts_with('0') {
            true
        } else if id.0.len() > 1
            && let Some((left, right)) = id.0.split_at_checked(id.0.len() / 2)
            && left == right
        {
            true
        } else {
            false
        }
    })
}

fn part2(input: &str) -> u64 {
    // Took 22 minutes 21,32 seconds (excluding breaks of around 40 minutes because of coworkers)
    sum_of_all_invalid_ids(input.parse().expect("Should parse fine"), |id| {
        if id.0.starts_with('0') {
            return true;
        }
        enum Search {
            None,
            Searching(String),
            FoundAtLeastTwice(String),
            Failed,
        }
        (1..=id.0.len() / 2)
            .filter(|chunk_size| (id.0.len() / chunk_size) * chunk_size == id.0.len())
            .map(|chunk_size| {
                id.0.chars()
                    .chunks(chunk_size)
                    .into_iter()
                    .fold(Search::None, |acc, value| match acc {
                        Search::None => Search::Searching(value.collect()),
                        Search::Searching(current) => {
                            if current == value.collect::<String>() {
                                Search::FoundAtLeastTwice(current)
                            } else {
                                Search::Failed
                            }
                        }
                        Search::FoundAtLeastTwice(current) => {
                            if current == value.collect::<String>() {
                                Search::FoundAtLeastTwice(current)
                            } else {
                                Search::Failed
                            }
                        }
                        Search::Failed => Search::Failed,
                    })
            })
            .any(|search| matches!(search, Search::FoundAtLeastTwice(_)))
    })
}

fn sum_of_all_invalid_ids<F>(ranges: IdRanges, invalid_predicate: F) -> u64
where
    F: Fn(&Id) -> bool + Copy,
{
    ranges.sum_by(invalid_predicate).expect("Should calculate")
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct IdRanges(Box<[IdRange]>);

impl IdRanges {
    fn sum_by<F>(&self, predicate: F) -> Result<u64, SumByIdRangesError>
    where
        F: Fn(&Id) -> bool + Copy,
    {
        self.0
            .iter()
            .enumerate()
            .map(|(index, range)| {
                range
                    .sum_by(predicate)
                    .map_err(|error| SumByIdRangesError::InvalidCountForRange {
                        index,
                        source: error,
                    })
            })
            .try_fold(0u64, |acc, val| val.map(|val| val + acc))
    }
}

#[derive(thiserror::Error, Debug)]
enum SumByIdRangesError {
    #[error("Failed to get invalid count of range at index '{index}': {source}")]
    InvalidCountForRange {
        index: usize,
        source: SumByIdRangeError,
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
    fn sum_by<F>(&self, predicate: F) -> Result<u64, SumByIdRangeError>
    where
        F: Fn(&Id) -> bool,
    {
        Ok(self
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(index, current)| {
                current
                    .map(|id| predicate(&id).then_some((index, id)))
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(index, id)| {
                id.0.parse::<u64>()
                    .map_err(|error| SumByIdRangeError::Parse {
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
enum SumByIdRangeError {
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

    #[test]
    fn test_part2() {
        // Arrange
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        // Act
        let part1 = part2(input);

        // Assert
        assert_eq!(part1, 4174379265);
    }
}
