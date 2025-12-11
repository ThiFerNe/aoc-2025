use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
}

const INPUT: &str = include_str!("../input/input.day10");

fn part1(input: &str) -> u64 {
    // Took 1 hour 54 minutes 40,86 seconds
    determine_fewest_button_presses_to_configure_indicator_lights(
        input.parse().expect("Should parse"),
    )
}

fn determine_fewest_button_presses_to_configure_indicator_lights(manual: Manual) -> u64 {
    manual
        .0
        .iter()
        .map(|machine_description| {
            #[derive(Debug, Clone)]
            struct State {
                state: Box<[IndicatorLightState]>,
                _predecessor: Option<Rc<State>>,
                distance_to_start: u64,
                // heuristic_distance_to_target: u64,
            }
            impl State {
                fn new(
                    state: Box<[IndicatorLightState]>,
                    distance_to_start: u64,
                    // heuristic_distance_to_target: u64,
                ) -> Self {
                    Self {
                        state,
                        _predecessor: None,
                        distance_to_start,
                        // heuristic_distance_to_target,
                    }
                }

                fn new_predecessor(
                    state: Box<[IndicatorLightState]>,
                    predecessor: Rc<State>,
                    distance_to_start: u64,
                    // heuristic_distance_to_target: u64,
                ) -> Self {
                    Self {
                        state,
                        _predecessor: Some(predecessor),
                        distance_to_start,
                        // heuristic_distance_to_target,
                    }
                }

                fn heuristic_path_length(&self) -> u64 {
                    self.distance_to_start // + self.heuristic_distance_to_target
                }
            }
            impl PartialEq for State {
                fn eq(&self, other: &Self) -> bool {
                    self.state == other.state
                }
            }
            impl Eq for State {}
            impl Hash for State {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    self.state.hash(state);
                }
            }
            impl PartialOrd for State {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Ord for State {
                fn cmp(&self, other: &Self) -> Ordering {
                    self.heuristic_path_length()
                        .cmp(&other.heuristic_path_length())
                        .reverse()
                }
            }

            /*fn distance(left: &[IndicatorLightState], right: &[IndicatorLightState]) -> u64 {
                assert_eq!(left.len(), right.len());
                left.iter()
                    .zip(right.iter())
                    .filter(|(l, r)| l != r)
                    .count() as u64
            }*/

            let mut closed_states = HashSet::<Rc<State>>::new();
            let mut open_states = BinaryHeap::<Rc<State>>::from([{
                let state = vec![
                    IndicatorLightState::Off;
                    machine_description.indicator_light_diagram.target.len()
                ]
                .into_boxed_slice();
                /*let distance_to_target =
                distance(&machine_description.indicator_light_diagram.target, &state);*/
                Rc::new(State::new(state, 0 /* distance_to_target */))
            }]);

            while let Some(current_node) = open_states.pop() {
                closed_states.insert(Rc::clone(&current_node));
                if current_node.state == machine_description.indicator_light_diagram.target {
                    break;
                }
                let successors = machine_description.button_wiring_schematics.iter().map(
                    |button_wiring_schematic| {
                        let mut state = current_node.state.clone();
                        for target_indicator_light in &button_wiring_schematic.0 {
                            state[target_indicator_light.index].invert();
                        }
                        /*let distance_to_target =
                        distance(&machine_description.indicator_light_diagram.target, &state);*/
                        Rc::new(State::new_predecessor(
                            state,
                            Rc::clone(&current_node),
                            current_node.distance_to_start + 1,
                            //distance_to_target,
                        ))
                    },
                );
                for successor in successors {
                    if closed_states.contains(&successor) {
                        continue;
                    }
                    if let Some(open_state) = open_states
                        .iter()
                        .find(|state| state.state == successor.state)
                    {
                        if open_state.distance_to_start > successor.distance_to_start {
                            open_states.retain(|open_state| open_state.state != successor.state);
                            open_states.push(successor);
                        }
                    } else {
                        open_states.push(successor);
                    }
                }
            }

            closed_states
                .iter()
                .find(|s| s.state == machine_description.indicator_light_diagram.target)
                .expect("Should find a way")
                .distance_to_start
        })
        .sum()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Manual(Box<[MachineDescription]>);

impl FromStr for Manual {
    type Err = ParseManualError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(FromStr::from_str).collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseManualError {
    #[error("Failed to parse machine description: {0}")]
    ParseMachineDescription(#[from] ParseMachineDescriptionError),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct MachineDescription {
    indicator_light_diagram: IndicatorLightDiagram,
    button_wiring_schematics: Box<[ButtonWiringSchematic]>,
    joltage_requirements: JoltageRequirements,
}

impl FromStr for MachineDescription {
    type Err = ParseMachineDescriptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rest = s
            .strip_prefix('[')
            .ok_or(ParseMachineDescriptionError::MissingIndicatorLightStartDelimiter)?;
        let end_index_indicator_light_diagram = rest
            .find(']')
            .ok_or(ParseMachineDescriptionError::MissingIndicatorLightEndDelimiter)?;
        let indicator_light_diagram = rest[..end_index_indicator_light_diagram].parse()?;

        let mut button_wiring_schematics = Vec::new();
        loop {
            let Some(next_button_wiring_schematic_start_index) = rest.find('(') else {
                break;
            };
            let next_button_wiring_schematic_end_index = rest
                [next_button_wiring_schematic_start_index..]
                .find(')')
                .ok_or(ParseMachineDescriptionError::MissingButtonWiringSchematicEndDelimiter)?;
            button_wiring_schematics.push(
                rest[next_button_wiring_schematic_start_index + 1
                    ..next_button_wiring_schematic_start_index
                        + next_button_wiring_schematic_end_index]
                    .parse()?,
            );
            rest = &rest[next_button_wiring_schematic_start_index
                + next_button_wiring_schematic_end_index..];
        }
        if button_wiring_schematics.is_empty() {
            return Err(ParseMachineDescriptionError::NoButtonWiringSchematics);
        }

        let joltage_requirements_start_index = rest
            .find('{')
            .ok_or(ParseMachineDescriptionError::MissingJoltageRequirementsStartDelimiter)?;
        let joltage_requirements_end_index = rest[joltage_requirements_start_index..]
            .find('}')
            .ok_or(ParseMachineDescriptionError::MissingJoltageRequirementsEndDelimiter)?;
        let joltage_requirements = rest[joltage_requirements_start_index + 1
            ..joltage_requirements_start_index + joltage_requirements_end_index]
            .parse()?;

        Ok(Self {
            indicator_light_diagram,
            button_wiring_schematics: button_wiring_schematics.into_boxed_slice(),
            joltage_requirements,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseMachineDescriptionError {
    #[error("Missing indicator light start delimiter")]
    MissingIndicatorLightStartDelimiter,
    #[error("Missing indicator light end delimiter")]
    MissingIndicatorLightEndDelimiter,
    #[error("Failed to parse indicator light diagram: {0}")]
    ParseIndicatorLight(#[from] ParseIndicatorLightDiagramError),
    #[error("Missing button wiring schematic end delimiter")]
    MissingButtonWiringSchematicEndDelimiter,
    #[error("Failed to parse button wiring schematic: {0}")]
    ParseButtonWiringSchematic(#[from] ParseButtonWiringSchematicError),
    #[error("No button wiring schematics found")]
    NoButtonWiringSchematics,
    #[error("Missing joltage requirements start delimiter")]
    MissingJoltageRequirementsStartDelimiter,
    #[error("Missing joltage requirements end delimiter")]
    MissingJoltageRequirementsEndDelimiter,
    #[error("Failed to parse button wiring schematic: {0}")]
    ParseJoltageRequirements(#[from] ParseJoltageRequirementsError),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct IndicatorLightDiagram {
    target: Box<[IndicatorLightState]>,
}

impl FromStr for IndicatorLightDiagram {
    type Err = ParseIndicatorLightDiagramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            target: s
                .chars()
                .map(IndicatorLightState::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseIndicatorLightDiagramError {
    #[error("Failed to parse indicator light state: {0}")]
    ParseIndicatorLight(#[from] ParseIndicatorLightStateError),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum IndicatorLightState {
    On,
    Off,
}

impl IndicatorLightState {
    fn invert(&mut self) {
        *self = match self {
            IndicatorLightState::On => IndicatorLightState::Off,
            IndicatorLightState::Off => IndicatorLightState::On,
        };
    }

    fn parse(c: char) -> Result<Self, ParseIndicatorLightStateError> {
        match c {
            '.' => Ok(IndicatorLightState::Off),
            '#' => Ok(IndicatorLightState::On),
            _ => Err(ParseIndicatorLightStateError::Unknown),
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseIndicatorLightStateError {
    #[error("Unknown indicator light state")]
    Unknown,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct ButtonWiringSchematic(Box<[TargetIndicatorLight]>);

impl FromStr for ButtonWiringSchematic {
    type Err = ParseButtonWiringSchematicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseButtonWiringSchematicError {
    #[error("Failed to parse target indicator light: {0}")]
    ParseTargetIndicatorLight(#[from] ParseTargetIndicatorLightError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct TargetIndicatorLight {
    index: usize,
}

impl FromStr for TargetIndicatorLight {
    type Err = ParseTargetIndicatorLightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { index: s.parse()? })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseTargetIndicatorLightError {
    #[error("Failed to parse value: {0}")]
    Parse(#[from] ParseIntError),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct JoltageRequirements(Box<[u64]>);

impl FromStr for JoltageRequirements {
    type Err = ParseJoltageRequirementsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseJoltageRequirementsError {
    #[error("Failed to parse joltage requirement: {0}")]
    Parse(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 7);
    }
}
