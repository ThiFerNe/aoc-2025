use crate::count::{bounded, bounded_inclusive};
use crate::solver::{EquationsCount, SystemOfLinearEquations, VariablesCount};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::once;
use std::num::ParseIntError;
use std::rc::Rc;
use std::str::FromStr;
use std::time::{Duration, Instant};

fn main() {
    aoc_2025::aoc!(INPUT, part1, part2);
}

const INPUT: &str = include_str!("../input/input.day10");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Took 1 hour 54 minutes 40,86 seconds
    determine_fewest_button_presses_to_configure_indicator_lights(
        input.parse().expect("Should parse"),
    )
}

#[cfg(feature = "part2")]
fn part2(input: &str) -> u64 {
    // Took 5 hours 24 minutes 51,14 seconds with multiple breaks and multiple days
    determine_fewest_button_presses_to_configure_joltage_levels(
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

mod solver {
    /*
    Inspired by:
    - https://www.reddit.com/r/adventofcode/comments/1pl8nsa/comment/ntqtuus/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    - https://github.com/icub3d/advent-of-code/blob/main/aoc_2025/src/bin/day10.rs
    */
    use std::fmt::{Display, Formatter};

    use nnn::{NNNewType, nnn};

    pub struct SystemOfLinearEquations<State: sealed::SOLE> {
        data: Vec<Vec<f64>>,
        state: State,
    }

    impl SystemOfLinearEquations<Initial> {
        pub fn new_with_fn<F, G>(
            equations_count: EquationsCount,
            variables_count: VariablesCount,
            variables_fn: F,
            results_fn: G,
        ) -> Self
        where
            F: Fn(EquationIndex, VariableIndex) -> f64,
            G: Fn(EquationIndex) -> f64,
        {
            let mut data = Vec::with_capacity(equations_count.into_inner());
            for equation_index in 0..equations_count.into_inner() {
                let mut equation = Vec::with_capacity(1usize + variables_count.into_inner());
                for variable_index in 0..variables_count.into_inner() {
                    equation.push(variables_fn(
                        EquationIndex(equation_index),
                        VariableIndex(variable_index),
                    ));
                }
                equation.push(results_fn(EquationIndex(equation_index)));
                data.push(equation);
            }
            Self {
                data,
                state: Initial,
            }
        }

        pub fn gaussian_elimination(self) -> SystemOfLinearEquations<Eliminated> {
            let SystemOfLinearEquations { data, state: _ } = self;
            let mut output = SystemOfLinearEquations {
                data,
                state: Eliminated {
                    dependent_variables: Vec::new(),
                    independent_variables: Vec::new(),
                },
            };

            const EPSILON: f64 = 1e-9;

            // https://en.wikipedia.org/wiki/Gaussian_elimination
            let mut current_row_index = 0;
            let mut current_column_index = 0;

            while current_row_index < output.data.len()
                && current_column_index < output.data[current_row_index].len().saturating_sub(1)
            {
                // Find best remaining row, based on biggest variable for the current row index
                let (best_row_index, best_value) = output
                    .data
                    .iter()
                    .enumerate()
                    .skip(current_row_index)
                    .map(|(row_index, row)| (row_index, row[current_column_index].abs()))
                    .max_by(|(_, row_a), (_, row_b)| {
                        row_a
                            .partial_cmp(row_b)
                            .expect("Should be able to compare floating point numbers")
                    })
                    .expect("Should not be empty as at least current row should be found");

                // If the best value is zero, this is a free variable:
                if best_value < EPSILON {
                    output
                        .state
                        .independent_variables
                        .push(VariableIndex(current_column_index));
                    current_column_index += 1;
                    continue;
                }

                // Swap rows and mark this column as dependent:
                output.data.swap(current_row_index, best_row_index);
                output
                    .state
                    .dependent_variables
                    .push(VariableIndex(current_column_index));

                // Normalize current row:
                let current_row_base_value = output.data[current_row_index][current_column_index];
                for variable_value in &mut output.data[current_row_index][current_column_index..] {
                    *variable_value /= current_row_base_value;
                }

                // Eliminate this column in all other rows:
                for row_index in 0..output.data.len() {
                    if row_index != current_row_index {
                        let factor = output.data[row_index][current_column_index];
                        if factor.abs() > EPSILON {
                            let current_row =
                                output.data[current_row_index][current_column_index..].to_vec();
                            output.data[row_index][current_column_index..]
                                .iter_mut()
                                .zip(current_row.iter())
                                .for_each(|(row_value, current_row_value)| {
                                    *row_value -= factor * current_row_value;
                                });
                        }
                    }
                }

                current_row_index += 1;
                current_column_index += 1;
            }

            output.state.independent_variables.extend(
                (current_column_index
                    ..output
                        .data
                        .first()
                        .map(|first_row| first_row.len().saturating_sub(1))
                        .unwrap_or(current_column_index))
                    .map(VariableIndex),
            );

            output
        }
    }

    impl SystemOfLinearEquations<Eliminated> {
        pub fn dependent_variables(&self) -> &[VariableIndex] {
            &self.state.dependent_variables
        }

        pub fn independent_variables(&self) -> &[VariableIndex] {
            &self.state.independent_variables
        }

        pub fn calculate_dependents(&self, independent_variable_values: &[f64]) -> Box<[f64]> {
            assert_eq!(
                independent_variable_values.len(),
                self.state.independent_variables.len()
            );
            (0..self.state.dependent_variables.len())
                .map(|equation_index| {
                    self.state
                        .independent_variables
                        .iter()
                        .zip(independent_variable_values)
                        .fold(
                            *self.data[equation_index]
                                .last()
                                .expect("Should get result value"),
                            |result, (independent_variable_index, independent_variable_value)| {
                                result
                                    - self.data[equation_index][independent_variable_index.0]
                                        * *independent_variable_value
                            },
                        )
                })
                .collect()
        }
    }

    fn maximum_value_str_length(data: &Vec<Vec<f64>>) -> usize {
        data.iter()
            .flat_map(|row| row.iter().map(|value| value.to_string().len()))
            .max()
            .unwrap_or(0)
    }

    fn display_initial(data: &Vec<Vec<f64>>, f: &mut Formatter<'_>) -> std::fmt::Result {
        let maximum_value_str_length = maximum_value_str_length(data);
        for row in data {
            for (column_index, cell) in row.iter().enumerate() {
                if column_index + 1 == row.len() {
                    write!(f, " | {cell:>width$}", width = maximum_value_str_length)?;
                } else {
                    write!(f, "{cell:>width$} ", width = maximum_value_str_length)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }

    impl Display for SystemOfLinearEquations<Initial> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            display_initial(&self.data, f)
        }
    }

    impl Display for SystemOfLinearEquations<Eliminated> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            display_initial(&self.data, f)?;
            let maximum_value_str_length = maximum_value_str_length(&self.data);
            let column_count = self.data.first().map(|row| row.len()).unwrap_or(0);
            for column_index in 0..column_count {
                let str = if self
                    .state
                    .independent_variables
                    .iter()
                    .any(|v| v.0 == column_index)
                {
                    "I"
                } else if self
                    .state
                    .dependent_variables
                    .iter()
                    .any(|v| v.0 == column_index)
                {
                    "D"
                } else {
                    "?"
                };
                if column_index + 1 == column_count {
                    write!(f, " | {str:>width$}", width = maximum_value_str_length)?;
                } else {
                    write!(f, "{str:>width$} ", width = maximum_value_str_length)?;
                }
            }
            Ok(())
        }
    }

    #[nnn(
        derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug),
        nnn_derive(TryFrom),
        validators(min = 1)
    )]
    pub struct EquationsCount(usize);

    #[nnn(
        derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug),
        nnn_derive(TryFrom),
        validators(min = 1)
    )]
    pub struct VariablesCount(usize);

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub struct EquationIndex(pub usize);

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub struct VariableIndex(pub usize);

    mod sealed {
        pub trait SOLE {}
    }

    pub struct Initial;
    impl sealed::SOLE for Initial {}

    pub struct Eliminated {
        dependent_variables: Vec<VariableIndex>,
        independent_variables: Vec<VariableIndex>,
    }
    impl sealed::SOLE for Eliminated {}
}

mod count {
    use num_traits::{One, Zero};
    use std::ops::Add;

    pub fn bounded<T>(bounds: impl IntoIterator<Item = T>) -> BoundedIterator<T>
    where
        T: One + Zero + Add + PartialOrd + Clone,
    {
        let bounds: Box<[T]> = bounds.into_iter().collect();
        let bounds_len = bounds.len();
        BoundedIterator {
            bounds,
            next: Some(vec![T::zero(); bounds_len].into_boxed_slice()),
        }
    }

    pub fn bounded_inclusive<T>(bounds: impl IntoIterator<Item = T>) -> BoundedIterator<T>
    where
        T: One + Zero + Add + PartialOrd + Clone,
    {
        let bounds: Box<[T]> = bounds.into_iter().map(|v| v + T::one()).collect();
        let bounds_len = bounds.len();
        BoundedIterator {
            bounds,
            next: Some(vec![T::zero(); bounds_len].into_boxed_slice()),
        }
    }

    pub struct BoundedIterator<T>
    where
        T: One + Zero + Add + PartialOrd + Clone,
    {
        bounds: Box<[T]>,
        next: Option<Box<[T]>>,
    }

    impl<T> Iterator for BoundedIterator<T>
    where
        T: One + Zero + Add + PartialOrd + Clone,
    {
        type Item = Box<[T]>;

        fn next(&mut self) -> Option<Self::Item> {
            let maybe_current = self.next.take();
            if let Some(current) = &maybe_current {
                let mut next = current.clone();
                let Some(mut next_index) = next.len().checked_sub(1) else {
                    return maybe_current;
                };
                next[next_index] = next[next_index].clone() + T::one();
                while next[next_index] >= self.bounds[next_index] {
                    next[next_index] = T::zero();
                    let Some(after_next_index) = next_index.checked_sub(1) else {
                        return maybe_current;
                    };
                    next[after_next_index] = next[after_next_index].clone() + T::one();
                    next_index = after_next_index;
                }
                self.next = Some(next);
            }
            maybe_current
        }
    }
}

fn determine_fewest_button_presses_to_configure_joltage_levels(manual: Manual) -> u64 {
    let machine_count = manual.0.len();
    manual
        .0
        .iter()
        .enumerate()
        .map(|(index, machine_description)| {
            // TODO println!();
            // TODO println!("MACHINE: #{index} / {machine_count}");

            let system = SystemOfLinearEquations::new_with_fn(
                EquationsCount::try_from(machine_description.joltage_requirements.0.len()).unwrap(),
                VariablesCount::try_from(machine_description.button_wiring_schematics.len())
                    .unwrap(),
                |joltage_index, button_index| {
                    if machine_description.button_wiring_schematics[button_index.0]
                        .0
                        .iter()
                        .any(|wiring_target| wiring_target.index == joltage_index.0)
                    {
                        1.
                    } else {
                        0.
                    }
                },
                |joltage_index| {
                    machine_description.joltage_requirements.0[joltage_index.0].0 as f64
                },
            )
            .gaussian_elimination();

            // TODO println!("{:?}", system.independent_variables());
            let maximum_presses_per_button = system
                .independent_variables()
                .iter()
                .filter_map(|button_index| {
                    machine_description.button_wiring_schematics[button_index.0]
                        .0
                        .iter()
                        .map(|wiring_target| {
                            machine_description.joltage_requirements.0[wiring_target.index].0
                        })
                        .max()
                })
                .collect::<Box<[_]>>();
            // TODO println!("maximum_presses_per_button = {maximum_presses_per_button:?}");

            let solution = bounded_inclusive(maximum_presses_per_button)
                .filter_map(|independents| {
                    // TODO println!("{independents:?}");
                    let dependants = system.calculate_dependents(
                        &independents.iter().map(|v| *v as f64).collect::<Box<[_]>>(),
                    );
                    let dependants = {
                        let mut new_dependants = Vec::with_capacity(dependants.len());
                        for dependent in dependants {
                            let rounded = dependent.round();
                            if dependent < -1e-9 || (dependent - rounded).abs() > 1e-9 {
                                return None;
                            } else {
                                new_dependants.push(rounded as u64);
                            }
                        }
                        new_dependants.into_boxed_slice()
                    };

                    let all = system
                        .independent_variables()
                        .iter()
                        .zip(independents.into_iter())
                        .chain(
                            system
                                .dependent_variables()
                                .iter()
                                .zip(dependants.into_iter()),
                        )
                        .sorted_by_key(|(index, _)| index.0)
                        .collect::<Box<[_]>>();

                    // TODO println!("{all:?}");

                    Some(all)
                })
                .min_by_key(|a| a.iter().map(|b| b.1).sum::<u64>())
                .expect("Should get at least one");

            // TODO println!("solution = {solution:?}");

            solution.iter().map(|b| b.1).sum::<u64>()
        })
        .sum()
}

fn determine_fewest_button_presses_to_configure_joltage_levels_c(manual: Manual) -> u64 {
    manual
        .0
        .iter()
        .enumerate()
        .map(|(index, machine_description)| {
            println!("MACHINE_DESCRIPTION #{index}");
            let maximum_press_count = machine_description
                .button_wiring_schematics
                .iter()
                .map(|button_wiring_schematic| {
                    let mut joltage_auxiliary_calculation =
                        vec![Joltage(0); machine_description.joltage_requirements.0.len()];
                    fn is_under_or_equal(this: &[Joltage], that: &[Joltage]) -> bool {
                        assert_eq!(this.len(), that.len());
                        this.iter()
                            .zip(that.iter())
                            .all(|(this_joltage, that_joltage)| this_joltage.0 <= that_joltage.0)
                    }
                    let mut counter = 0u64;
                    while is_under_or_equal(
                        &joltage_auxiliary_calculation,
                        &machine_description.joltage_requirements.0,
                    ) {
                        for button_wiring_schematic_index in &button_wiring_schematic.0 {
                            joltage_auxiliary_calculation[button_wiring_schematic_index.index].0 +=
                                1;
                        }
                        counter += 1;
                    }
                    counter
                })
                .collect::<Box<[_]>>();
            println!("maximum_press_count={maximum_press_count:?}");

            let mut fitting: Vec<Box<[u64]>> = Vec::new();
            let mut current_press_combination =
                vec![0; machine_description.button_wiring_schematics.len()];

            let mut last = Instant::now();
            'outer: loop {
                if last.elapsed().as_secs_f64() > 2.0 {
                    println!("current_press_combination={current_press_combination:?}");
                    last = Instant::now();
                }
                let mut result_after_pressing =
                    vec![Joltage(0); machine_description.joltage_requirements.0.len()];
                for (index, count) in current_press_combination.iter().enumerate() {
                    for _ in 0..*count {
                        for wiring_target in &machine_description.button_wiring_schematics[index].0
                        {
                            result_after_pressing[wiring_target.index].0 += 1;
                        }
                    }
                }

                if &result_after_pressing[..] == &machine_description.joltage_requirements.0[..] {
                    fitting.push(current_press_combination.clone().into_boxed_slice());
                }

                for index in 0..result_after_pressing.len() {
                    if result_after_pressing[index].0
                        > machine_description.joltage_requirements.0[index].0
                    {
                        machine_description
                            .button_wiring_schematics
                            .iter()
                            .enumerate()
                            .filter(|bws| bws.1.0.iter().any(|wt| wt.index == index))
                            .for_each(|(index, _)| {
                                current_press_combination[index] =
                                    maximum_press_count[index].saturating_sub(1)
                            });
                    }
                }

                // Update Loop Variable
                let mut check_index = current_press_combination.len() - 1;
                current_press_combination[check_index] += 1;
                while current_press_combination[check_index] > maximum_press_count[check_index] {
                    current_press_combination[check_index] = 0;
                    if check_index > 0 {
                        current_press_combination[check_index - 1] += 1;
                    } else {
                        break 'outer;
                    }
                    check_index -= 1;
                }
            }

            fitting
                .into_iter()
                .map(|variant| variant.into_iter().sum::<u64>())
                .min()
                .expect("Should find at least one")
        })
        .sum()
}

fn determine_fewest_button_presses_to_configure_joltage_levels_b(manual: Manual) -> u64 {
    manual
        .0
        .iter()
        .enumerate()
        .map(|(index, machine_description)| {
            println!("====> MACHINE_DESCRIPTION #{index}");
            #[derive(Debug, Clone)]
            struct State {
                state: Box<[Joltage]>,
                _predecessor: Option<Rc<State>>,
                distance_to_start: u64,
                heuristic_distance_to_target: u64,
            }
            impl State {
                fn new(
                    state: Box<[Joltage]>,
                    distance_to_start: u64,
                    heuristic_distance_to_target: u64,
                ) -> Self {
                    Self {
                        state,
                        _predecessor: None,
                        distance_to_start,
                        heuristic_distance_to_target,
                    }
                }

                fn new_predecessor(
                    state: Box<[Joltage]>,
                    predecessor: Rc<State>,
                    distance_to_start: u64,
                    heuristic_distance_to_target: u64,
                ) -> Self {
                    Self {
                        state,
                        _predecessor: Some(predecessor),
                        distance_to_start,
                        heuristic_distance_to_target,
                    }
                }

                fn heuristic_path_length(&self) -> u64 {
                    /*self.distance_to_start + */
                    self.heuristic_distance_to_target
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

            fn heuristic_distance(left: &[Joltage], right: &[Joltage]) -> u64 {
                assert_eq!(left.len(), right.len());
                left.iter()
                    .zip(right.iter())
                    .map(|(l, r)| l.0.abs_diff(r.0))
                    .sum::<u64>()
            }

            let mut closed_states = HashSet::<Rc<State>>::new();
            let mut open_states = BinaryHeap::<Rc<State>>::from([{
                let state = vec![Joltage(0); machine_description.joltage_requirements.0.len()]
                    .into_boxed_slice();
                let heuristic_distance_to_target =
                    heuristic_distance(&machine_description.joltage_requirements.0, &state);
                Rc::new(State::new(state, 0, heuristic_distance_to_target))
            }]);

            let mut last_output = Instant::now();

            while let Some(current_node) = open_states.pop() {
                if last_output.elapsed() > Duration::from_secs_f64(2.0) {
                    //println!("{}", current_node.heuristic_distance_to_target);
                    println!(
                        "closed_states:{} open_states:{} current_node:({}-{}):{:?}",
                        closed_states.len(),
                        open_states.len(),
                        current_node.distance_to_start,
                        current_node.heuristic_distance_to_target,
                        current_node.state
                    );
                    //println!("T {:?}", machine_description.joltage_requirements.0);
                    last_output = Instant::now();
                }
                closed_states.insert(Rc::clone(&current_node));
                if current_node.state == machine_description.joltage_requirements.0 {
                    break;
                }
                let successors = machine_description
                    .button_wiring_schematics
                    .iter()
                    .map(|button_wiring_schematic| {
                        let mut state = current_node.state.clone();
                        for wiring_target in &button_wiring_schematic.0 {
                            state[wiring_target.index].0 += 1;
                        }
                        let heuristic_distance_to_target =
                            heuristic_distance(&machine_description.joltage_requirements.0, &state);
                        Rc::new(State::new_predecessor(
                            state,
                            Rc::clone(&current_node),
                            current_node.distance_to_start + 1,
                            heuristic_distance_to_target,
                        ))
                    })
                    .filter(|successor| {
                        successor
                            .state
                            .iter()
                            .zip(machine_description.joltage_requirements.0.iter())
                            .all(|(left, right)| left.0 <= right.0)
                    });
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
                .find(|s| s.state == machine_description.joltage_requirements.0)
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
struct ButtonWiringSchematic(Box<[WiringTarget]>);

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
    ParseParseWiringTarget(#[from] ParseWiringTargetError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct WiringTarget {
    index: usize,
}

impl FromStr for WiringTarget {
    type Err = ParseWiringTargetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { index: s.parse()? })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseWiringTargetError {
    #[error("Failed to parse value: {0}")]
    Parse(#[from] ParseIntError),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct JoltageRequirements(Box<[Joltage]>);

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
    #[error("Failed to parse joltage: {0}")]
    ParseJoltage(#[from] ParseJoltageError),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Joltage(u64);

impl FromStr for Joltage {
    type Err = ParseJoltageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseJoltageError {
    #[error("Failed to parse number: {0}")]
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

    #[test]
    fn test_part2() {
        // Arrange
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        // Act
        let part2 = part2(input);

        // Assert
        assert_eq!(part2, 33);
    }
}
