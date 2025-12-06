use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
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

#[derive(Clone, Eq, PartialEq, Debug)]
struct Bank(Box<[Battery]>);

impl Bank {
    fn joltage_rating(&self) -> u64 {
        let joltage_rating = self
            .0
            .iter()
            .filter_map(|battery| battery.active.then_some(battery.joltage_rating))
            .map(|joltage_rating| (b'0' + joltage_rating as u8) as char)
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
}
