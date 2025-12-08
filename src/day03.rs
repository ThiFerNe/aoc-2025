#[cfg(feature = "part2")]
use std::cmp::Ordering;
#[cfg(feature = "part2")]
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day03");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 26 minutes 28,26 seconds (excluding breaks of around 7 minutes because of cat)
    let mut banks = input.parse::<Banks>().expect("Should parse");
    banks.0.iter_mut().for_each(|bank| {
        let mut max_activated_bank = None;
        for first_index in 0..(bank.0.len() - 1) {
            for second_index in (first_index + 1)..bank.0.len() {
                let mut current_bank = bank.clone();
                current_bank.0[first_index].active = true;
                current_bank.0[second_index].active = true;
                max_activated_bank = match max_activated_bank {
                    None => Some(current_bank),
                    Some(max_bank) => {
                        if current_bank.joltage_rating() > max_bank.joltage_rating() {
                            Some(current_bank)
                        } else {
                            Some(max_bank)
                        }
                    }
                };
            }
        }
        if let Some(max_activated_bank) = max_activated_bank {
            *bank = max_activated_bank;
        }
    });
    banks.joltage_rating()
}

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // In total 2 hours 41 minute 39,4 seconds
    //
    // Try #1: 1 hour 13 minutes 52,77 seconds
    // Try #1&#2: 1 hour 9 minutes 44,50 seconds
    // Checking for tips
    // Try #3 and solution: Last solution 18 minutes 2,13 seconds

    let mut banks = input.parse::<Banks>().expect("Should parse");
    banks.0.iter_mut().for_each(|bank| {
        let mut maximum_indices = Vec::new();
        while maximum_indices.len() < 12
            && maximum_indices
                .last()
                .map(|index| *index < bank.0.len())
                .unwrap_or(true)
        {
            let sub_index = maximum_indices
                .last()
                .copied()
                .map(|index| index + 1)
                .unwrap_or(0);
            let sub_len = bank.0.len() - sub_index - (12 - maximum_indices.len() - 1);
            let (maximum_index, _maximum_joltage) = bank
                .0
                .iter()
                .enumerate()
                .skip(sub_index)
                .take(sub_len)
                .rev()
                .map(|(index, battery)| (index, battery.joltage_rating))
                .max_by_key(|(_index, joltage_rating)| *joltage_rating)
                .expect("Should not have empty bank");
            maximum_indices.push(maximum_index);
        }
        for index in maximum_indices {
            bank.0[index].active = true;
        }
    });
    banks.joltage_rating()
}

#[cfg(feature = "part2")]
#[allow(dead_code)]
fn part2_try2(input: &str) -> u64 {
    let mut banks = input.parse::<Banks>().expect("Should parse");
    banks.0.iter_mut().for_each(|bank| {
        // Histogram
        // Choose upper digits where histogram sum is at least COUNT
        // go through each digit-index where digit is from upper histogram and recurse for the rest until COUNT is matched
        // take highest
        fn find_configurations(
            sub_bank: &[Battery],
            remaining_count: u64,
        ) -> impl Iterator<Item = Vec<bool>> {
            assert!(sub_bank.len() >= remaining_count as usize);
            let search_length = sub_bank.len() as u64 - (remaining_count - 1);
            let search_sub_bank = &sub_bank[..search_length as usize];

            let joltage_rating_histogram = search_sub_bank
                .iter()
                .map(|battery| battery.joltage_rating)
                .fold(HashMap::new(), |mut acc, joltage_rating| {
                    acc.entry(joltage_rating)
                        .and_modify(|c| *c += 1)
                        .or_insert(1u64);
                    acc
                });

            let mut remaining_count_search_variable = remaining_count.min(search_length);
            let minimum_search_joltage_rating = *joltage_rating_histogram
                .iter()
                .sorted_by(|a, b| Ord::cmp(&a.0, &b.0).reverse())
                .find(
                    |(_joltage_rating, count)| match remaining_count_search_variable.cmp(count) {
                        Ordering::Less | Ordering::Equal => true,
                        Ordering::Greater => {
                            remaining_count_search_variable -= *count;
                            false
                        }
                    },
                )
                .expect("Should have more or equal elements than searching for")
                .0;

            search_sub_bank
                .iter()
                .enumerate()
                .filter(move |(_index, battery)| {
                    battery.joltage_rating >= minimum_search_joltage_rating
                })
                .flat_map(move |(index, _battery)| {
                    let mut activation = Vec::with_capacity(index + 1);
                    activation.extend(std::iter::repeat_n(false, index));
                    activation.push(true);
                    if remaining_count <= 1 {
                        vec![activation].into_iter()
                    } else {
                        find_configurations(&sub_bank[(index + 1)..], remaining_count - 1)
                            .map(move |inner_activation| {
                                let mut activation = activation.clone();
                                activation.extend_from_slice(&inner_activation);
                                activation
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                    }
                })
        }

        *bank = find_configurations(&bank.0, 12)
            .map(|activations| {
                let mut cloned_bank = bank.clone();
                cloned_bank
                    .0
                    .iter_mut()
                    .zip(activations)
                    .for_each(|(bank, activity)| bank.active = activity);
                let rating = cloned_bank.joltage_rating();
                (cloned_bank, rating)
            })
            .reduce(|left, right| if left.1 >= right.1 { left } else { right })
            .expect("Should find best configuration")
            .0;
    });
    banks.joltage_rating()
}

#[cfg(feature = "part2")]
#[allow(dead_code)]
fn part2_try1(input: &str) -> u64 {
    let mut banks = input.parse::<Banks>().expect("Should parse");
    banks.0.iter_mut().for_each(|bank| {
        let mut batteries_by_index = bank.0.iter().enumerate().collect::<Vec<_>>();
        batteries_by_index.sort_by_key(|(_index, battery)| battery.joltage_rating);
        batteries_by_index.reverse();
        let (first_index, second_index, _joltage_rating) = batteries_by_index
            .into_iter()
            .filter_map(|(first_index, first_battery)| {
                bank.0
                    .iter()
                    .enumerate()
                    .skip(first_index + 1)
                    .max_by_key(|(_index, battery)| battery.joltage_rating)
                    .map(|(second_index, second_battery)| {
                        (
                            first_index,
                            second_index,
                            first_battery.joltage_rating * 10 + second_battery.joltage_rating,
                        )
                    })
            })
            .max_by_key(|(_first_index, _second_index, joltage_rating)| *joltage_rating)
            .expect("Should not be empty");
        bank.0[first_index].active = true;
        bank.0[second_index].active = true;
    });
    banks.joltage_rating()
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Banks(Box<[Bank]>);

impl Banks {
    fn joltage_rating(&self) -> u64 {
        self.0.iter().map(Bank::joltage_rating).sum()
    }
}

impl FromStr for Banks {
    type Err = ParseBanksError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .enumerate()
                .map(|(index, line)| {
                    line.parse().map_err(|error| ParseBanksError::ParseBank {
                        index,
                        source: error,
                    })
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseBanksError {
    #[error("Failed to parse bank at line index '{index}': {source}")]
    ParseBank {
        index: usize,
        source: ParseBankError,
    },
}

impl Display for Banks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(ToString::to_string).join("\n"))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Bank(Box<[Battery]>);

impl Bank {
    fn joltage_rating(&self) -> u64 {
        let joltage_rating = self
            .0
            .iter()
            .filter(|battery| battery.active)
            .map(|battery| Into::<char>::into(*battery))
            .collect::<String>();
        if joltage_rating.is_empty() {
            0
        } else {
            joltage_rating.parse().expect("Should parse as number")
        }
    }
}

impl FromStr for Bank {
    type Err = ParseBankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .enumerate()
                .map(|(index, character)| {
                    character
                        .try_into()
                        .map_err(|error| ParseBankError::ParseBattery {
                            index,
                            source: error,
                        })
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseBankError {
    #[error("Failed to parse battery at index '{index}': {source}")]
    ParseBattery {
        index: usize,
        source: ParseBatteryError,
    },
}

impl Display for Bank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            self.0.iter().map(ToString::to_string).collect::<String>(),
            self.joltage_rating()
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Battery {
    joltage_rating: u64,
    active: bool,
}

impl TryFrom<char> for Battery {
    type Error = ParseBatteryError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() {
            Ok(Self {
                joltage_rating: value as u64 - b'0' as u64,
                active: false,
            })
        } else {
            Err(ParseBatteryError::Unknown(value))
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseBatteryError {
    #[error("Failed to parse unknown Joltage Rating '{0}'")]
    Unknown(char),
}

#[allow(
    clippy::from_over_into,
    reason = "Do not have access to implement foreign Into for foreign char"
)]
impl Into<char> for Battery {
    fn into(self) -> char {
        (b'0' + self.joltage_rating as u8) as char
    }
}

impl Display for Battery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.joltage_rating,
            if self.active { "*" } else { "" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 357);
    }

    #[test]
    fn test_part2() {
        // Arrange
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        // Act
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 3121910778619);
    }
}
