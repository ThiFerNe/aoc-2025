use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day05");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 16 minutes 30,47 seconds
    Database::from_str(input)
        .expect("Should parse fine")
        .count_fresh_available_ingredients()
}

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 15 minutes 51 seconds
    Database::from_str(input)
        .expect("Should parse fine")
        .count_unique_fresh_ingredient_ids()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Database {
    fresh_ingredient_ranges: Box<[IngredientIdRange]>,
    available_ingredients: Box<[IngredientId]>,
}

impl Database {
    fn count_fresh_available_ingredients(&self) -> u64 {
        self.available_ingredients
            .iter()
            .filter(|available_ingredient| {
                self.fresh_ingredient_ranges
                    .iter()
                    .any(|fresh_ingredient_range| {
                        fresh_ingredient_range.contains(available_ingredient)
                    })
            })
            .count() as u64
    }

    fn count_unique_fresh_ingredient_ids(&self) -> u64 {
        let mut output = self.fresh_ingredient_ranges.to_vec();
        loop {
            let previous = output.len();
            output = output.into_iter().fold(Vec::new(), |mut acc, value| {
                let mut found = false;
                for range in &mut acc {
                    if let Some(merged) = range.merge(&value) {
                        *range = merged;
                        found = true;
                        break;
                    }
                }
                if !found {
                    acc.push(value);
                }
                acc
            });
            if previous == output.len() {
                break;
            }
        }
        output.iter().map(|range| range.len()).sum::<u64>()
    }
}

impl FromStr for Database {
    type Err = ParseDatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fresh_ingredient_ranges = Vec::new();
        let mut available_ingredients = Vec::new();
        let mut separator_line_found = false;
        for (index, line) in s.lines().enumerate() {
            if line.is_empty() {
                separator_line_found = true;
            } else if separator_line_found {
                available_ingredients.push(line.parse().map_err(|error| {
                    ParseDatabaseError::ParseAvailableIngredientId {
                        index,
                        source: error,
                    }
                })?);
            } else {
                fresh_ingredient_ranges.push(line.parse().map_err(|error| {
                    ParseDatabaseError::ParseFreshIngredientRange {
                        index,
                        source: error,
                    }
                })?);
            }
        }
        Ok(Self {
            fresh_ingredient_ranges: fresh_ingredient_ranges.into_boxed_slice(),
            available_ingredients: available_ingredients.into_boxed_slice(),
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseDatabaseError {
    #[error("Failed parsing line {index} as fresh ingredient range: {source}")]
    ParseFreshIngredientRange {
        index: usize,
        source: ParseIngredientRangeError,
    },
    #[error("Failed parsing line {index} as available ingredient id: {source}")]
    ParseAvailableIngredientId {
        index: usize,
        source: ParseIngredientIdError,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct IngredientIdRange {
    from: IngredientId,
    inclusive_to: IngredientId,
}

impl IngredientIdRange {
    fn contains(&self, id: &IngredientId) -> bool {
        self.from <= *id && *id <= self.inclusive_to
    }

    fn merge(&self, other: &Self) -> Option<Self> {
        if self.from <= other.inclusive_to && self.inclusive_to >= other.from {
            Some(Self {
                from: self.from.min(other.from),
                inclusive_to: self.inclusive_to.max(other.inclusive_to),
            })
        } else {
            None
        }
    }

    fn len(&self) -> u64 {
        self.inclusive_to.0 - self.from.0 + 1
    }
}

impl FromStr for IngredientIdRange {
    type Err = ParseIngredientRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s
            .split_once('-')
            .ok_or(ParseIngredientRangeError::MissingDelimiter)?;
        Ok(Self {
            from: from.parse().map_err(ParseIngredientRangeError::ParseFrom)?,
            inclusive_to: to
                .parse()
                .map_err(ParseIngredientRangeError::ParseInclusiveTo)?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseIngredientRangeError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Failed to parse from: {0}")]
    ParseFrom(#[source] ParseIngredientIdError),
    #[error("Failed to parse inclusive to: {0}")]
    ParseInclusiveTo(#[source] ParseIngredientIdError),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct IngredientId(u64);

impl FromStr for IngredientId {
    type Err = ParseIngredientIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseIngredientIdError {
    #[error("Failed to parse id as number: {0}")]
    Parse(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 3);
    }

    #[test]
    fn test_part2() {
        // Arrange
        let input = "3-5
10-14
16-20
12-18";

        // Act
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 14);
    }
}
