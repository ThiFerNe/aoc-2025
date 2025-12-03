use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
#[cfg(feature = "internal_timings")]
use std::time::Instant;

use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};

fn main() {
    #[cfg(feature = "internal_timings")]
    let start = Instant::now();
    #[cfg(any(feature = "part1", feature = "part2"))]
    let input = include_str!("../input/input.day02");
    #[cfg(feature = "part1")]
    {
        let part1 = part1(input);
        println!("Solution to part 1: {part1}");
    }
    #[cfg(feature = "part2")]
    {
        let part2 = part2(input);
        println!("Solution to part 1: {part2}");
    }
    #[cfg(feature = "internal_timings")]
    {
        let end = Instant::now();
        println!("duration: {:?}", end.duration_since(start));
    }
}

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 58 minutes 48,25 seconds (excluding breaks of around 60 minutes because of coworkers)
    sum_of_all_invalid_ids(input.parse().expect("Should parse fine"), |id| {
        if id.starts_with_zero() {
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

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 22 minutes 21,32 seconds (excluding breaks of around 40 minutes because of coworkers)
    sum_of_all_invalid_ids(input.parse().expect("Should parse fine"), |id| {
        if id.starts_with_zero() {
            return true;
        }
        enum Search<'a> {
            None,
            Searching(&'a [char]),
            FoundAtLeastTwice(&'a [char]),
            Failed,
        }
        (1..=id.0.len() / 2).rev().any(|chunk_size| {
            let result = (0..id.0.len())
                .step_by(chunk_size)
                .fold(Search::None, |acc, index| match acc {
                    Search::None => {
                        Search::Searching(&id.0[index..(index + chunk_size).min(id.0.len())])
                    }
                    Search::Searching(current) => {
                        if current == &id.0[index..(index + chunk_size).min(id.0.len())] {
                            Search::FoundAtLeastTwice(current)
                        } else {
                            Search::Failed
                        }
                    }
                    Search::FoundAtLeastTwice(current) => {
                        if current == &id.0[index..(index + chunk_size).min(id.0.len())] {
                            Search::FoundAtLeastTwice(current)
                        } else {
                            Search::Failed
                        }
                    }
                    Search::Failed => Search::Failed,
                });
            match result {
                Search::None | Search::Searching(_) | Search::Failed => false,
                Search::FoundAtLeastTwice(_) => true,
            }
        })
    })
}

fn sum_of_all_invalid_ids<F>(ranges: IdRanges, invalid_predicate: F) -> u64
where
    F: Fn(&Id) -> bool + Copy + Send + Sync,
{
    ranges.sum_by(invalid_predicate).expect("Should calculate")
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct IdRanges(Box<[IdRange]>);

impl IdRanges {
    fn sum_by<F>(&self, predicate: F) -> Result<u64, SumByIdRangesError>
    where
        F: Fn(&Id) -> bool + Copy + Send + Sync,
    {
        self.0
            .par_iter()
            .enumerate()
            .map(|(index, range)| {
                range
                    .sum_by(predicate)
                    .map_err(|error| SumByIdRangesError::SumByForRange {
                        index,
                        source: error,
                    })
            })
            .try_fold(|| 0u64, |acc, val| val.map(|val| val + acc))
            .try_reduce(|| 0, |left, right| Ok(left + right))
    }
}

#[derive(thiserror::Error, Debug)]
enum SumByIdRangesError {
    #[error("Failed to get sum by of range at index '{index}': {source}")]
    SumByForRange {
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
        F: Fn(&Id) -> bool + Send + Sync,
    {
        Ok(self
            .clone()
            .into_iter()
            .par_bridge()
            .filter_map(|current| current.map(|id| predicate(&id).then_some(id)).transpose())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|id| id.to_u64())
            .sum::<u64>())
    }
}

#[derive(thiserror::Error, Debug)]
enum SumByIdRangeError {
    #[error("Failed to iterate over id range: {0}")]
    Iterate(#[from] IterateIdRangeError),
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Id(Box<[char]>);

impl Id {
    fn next(&self) -> Result<Self, NextIdError> {
        let mut next = self.0.clone();
        let mut next_iter = next.iter_mut().rev();
        let mut current = next_iter.next().expect("Should not be empty");
        let mut carry_over = true;
        let mut overflow = false;

        while carry_over {
            carry_over = false;
            *current = match *current {
                '0' => '1',
                '1' => '2',
                '2' => '3',
                '3' => '4',
                '4' => '5',
                '5' => '6',
                '6' => '7',
                '7' => '8',
                '8' => '9',
                '9' => {
                    carry_over = true;
                    '0'
                }
                _ => unreachable!("All characters should be digits"),
            };
            if carry_over {
                if let Some(next_current) = next_iter.next() {
                    current = next_current;
                } else {
                    carry_over = false;
                    overflow = true;
                }
            }
        }
        if overflow {
            let mut new_next = Vec::with_capacity(next.len() + 1);
            new_next.push('1');
            new_next.extend_from_slice(&next);
            next = new_next.into_boxed_slice();
        }

        Ok(Self(next))
    }

    fn starts_with_zero(&self) -> bool {
        *self.0.first().expect("Should not be empty") == '0'
    }

    fn to_u64(&self) -> u64 {
        let mut output = 0;
        for (index, digit) in self.0.iter().rev().enumerate() {
            output += (*digit as u64 - '0' as u64) * 10u64.pow(index as u32);
        }
        output
    }
}

#[derive(thiserror::Error, Debug)]
enum NextIdError {
    #[error("Failed to parse internal value: {0}")]
    Parse(#[from] ParseIntError),
}

impl FromStr for Id {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Box<[_]>>();
        if chars.is_empty() {
            Err(ParseIdError::Empty)
        } else if chars.iter().all(|character| character.is_numeric()) {
            Ok(Self(chars))
        } else {
            Err(ParseIdError::NonNumeric)
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to parse id: {0}")]
enum ParseIdError {
    #[error("String is empty")]
    Empty,
    #[error("String contains non-numeric character")]
    NonNumeric,
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
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
