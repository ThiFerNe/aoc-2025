use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::Div;
use std::str::FromStr;

fn main() {
    aoc_2025::aoc!(INPUT, part1);
}

const INPUT: &str = include_str!("../input/input.day12");

#[cfg(feature = "part1")]
fn part1(input: &str) -> u64 {
    // Taken 1 hour 50 minutes 44,74 seconds
    let situation_summary = SituationSummary::from_str(input).expect("Should parse");
    let (max_shape_width, max_shape_length) = situation_summary
        .presents_shapes
        .iter()
        .map(|shape| (shape.width(), shape.length()))
        .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap();
    //let mut combination_repository = CombinationRepository::new();
    situation_summary
        .region_requirements
        .iter()
        .filter(|region_requirement| {
            // Possibilities
            // 1) Bounding Boxes fit, fine
            let available_box_count_width = region_requirement.size.width.div(max_shape_width);
            let available_box_count_length = region_requirement.size.length.div(max_shape_length);
            let box_count = available_box_count_width * available_box_count_length;

            let required_boxes = region_requirement.shape_quantity.iter().sum::<u64>();

            return required_boxes <= box_count;

            // 2) Check how they can be fit together and check if this then fits inside

            /*
            How do I fit two shapes together?
            - There maximum together edge length is max(width_a,length_a)+max(width_b,length_b)
            - We only have 3x3 shapes as initials
            - I will need 271 shapes for the first real input
            - I need to stack up / bake different shape combinations in increasing size (combining 0+0, 0+1, etc.), memoization?
            - There will be different combinations (Hashing!) HashMap::<[sorted_indices], HashSet<[combination_grid]>>
            - I have to do DFS and stop paths when they outgrow maximum size
            - When searching I will diverge into possible realities of (A) selected next shapes and
              (B) possible combination (as it does not have to be the best for this step which is best for all)
            */

            /*let a = ShapeCombination::generate_combinations(
                &situation_summary.presents_shapes[0],
                &situation_summary.presents_shapes[1],
            );
            for aa in a {
                println!();
                println!("{aa}");
            }*/

            // 3) brute force all variations
            true
        })
        .count() as u64
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct CombinationRepository(HashMap<ShapeIndices, HashSet<ShapeCombination>>);

impl CombinationRepository {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct ShapeIndices(Box<[ShapeIndex]>);

impl<T> From<T> for ShapeIndices
where
    T: IntoIterator<Item = ShapeIndex>,
{
    fn from(value: T) -> Self {
        let mut values = value.into_iter().collect::<Box<[_]>>();
        values.sort();
        Self(values)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct ShapeCombination {
    source: Box<[(ShapeIndex, ShapeTransformation)]>,
    field: Box<[Box<[Option<ShapeIndex>]>]>,
    field_offset_x: i64,
    field_offset_z: i64,
}

impl ShapeCombination {
    fn generate_combinations_all(present_shapes: &[PresentShape]) -> HashSet<Self> {
        todo!()
    }

    fn generate_combinations(
        present_shape_a: &PresentShape,
        present_shape_b: &PresentShape,
    ) -> HashSet<Self> {
        todo!()
    }

    fn width(&self) -> u64 {
        self.field.first().expect("Should not be empty").len() as u64
    }

    fn length(&self) -> u64 {
        self.field.len() as u64
    }

    fn check(&self, relative_x: i64, relative_z: i64) -> Option<ShapeIndex> {
        self.field
            .get((relative_z + self.field_offset_z) as usize)
            .and_then(|row| {
                row.get((relative_x + self.field_offset_x) as usize)
                    .copied()
            })
            .flatten()
    }
}

impl<T> From<T> for ShapeCombination
where
    T: IntoIterator<Item = (ShapeIndex, ShapeTransformation)>,
{
    fn from(value: T) -> Self {
        let mut values = value.into_iter().collect::<Box<[_]>>();
        values.sort();
        let mut field_offset_x = 0;
        let mut field_offset_z = 0;
        Self {
            source: values,
            field: Box::new([]),
            field_offset_x,
            field_offset_z,
        }
    }
}

impl Display for ShapeCombination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.field {
            for cell in row {
                match cell {
                    None => write!(f, ".")?,
                    Some(cell) => write!(f, "{cell}")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct ShapeTransformation(ShapeRotation, ShapeTranslation);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum ShapeRotation {
    None,
    Quarter,
    Half,
    ThreeQuarters,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct ShapeTranslation {
    x: i64,
    z: i64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct SituationSummary {
    presents_shapes: Box<[PresentShape]>,
    region_requirements: Box<[RegionRequirement]>,
}

impl FromStr for SituationSummary {
    type Err = ParseSituationSummaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = Vec::new();
        let mut current_section = String::new();

        for line in s.lines() {
            if line.is_empty() {
                if !current_section.is_empty() {
                    sections.push(current_section);
                    current_section = String::new();
                }
            } else {
                if !current_section.is_empty() {
                    current_section.push('\n');
                }
                current_section.push_str(line);
            }
        }
        if !current_section.is_empty() {
            sections.push(current_section);
        }

        let (last, rest) = sections
            .split_last()
            .expect("Should have at least two sections");
        println!("last:\n{last}");
        Ok(Self {
            presents_shapes: rest
                .iter()
                .map(|section| section.parse())
                .collect::<Result<_, _>>()?,
            region_requirements: last
                .lines()
                .enumerate()
                .map(|(index, line)| {
                    line.parse().map_err(|error| {
                        ParseSituationSummaryError::ParseRegionRequirements {
                            index,
                            source: error,
                        }
                    })
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseSituationSummaryError {
    #[error("Failed to parse present shape: {0}")]
    ParsePresentsShapes(#[from] ParsePresentShapeError),
    #[error("Failed to parse region requirements at index '{index}': {source}")]
    ParseRegionRequirements {
        index: usize,
        source: ParseRegionRequirementError,
    },
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct PresentShape {
    index: ShapeIndex,
    shape: Box<[Box<[ShapePart]>]>,
}

impl PresentShape {
    fn width(&self) -> u64 {
        self.shape.len() as u64
    }

    fn length(&self) -> u64 {
        self.shape
            .first()
            .expect("Should not be an empty present shape")
            .len() as u64
    }
}

impl FromStr for PresentShape {
    type Err = ParsePresentShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let index = lines
            .next()
            .ok_or(ParsePresentShapeError::Empty)?
            .strip_suffix(':')
            .ok_or(ParsePresentShapeError::MissingIndexSuffix)?
            .parse()?;
        Ok(Self {
            index,
            shape: lines
                .map(|line| {
                    line.chars()
                        .map(TryInto::try_into)
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParsePresentShapeError {
    #[error("Input is empty")]
    Empty,
    #[error("Missing index suffix")]
    MissingIndexSuffix,
    #[error("Failed to parse index: {0}")]
    ParseIndex(#[from] ParseShapeIndexError),
    #[error("Unknown shape part: {0}")]
    UnknownShapePart(#[from] ParseShapePartError),
}

#[derive(derive_more::Display, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct ShapeIndex(u64);

impl FromStr for ShapeIndex {
    type Err = ParseShapeIndexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseShapeIndexError {
    #[error("Failed to parse shape index: {0}")]
    Parse(#[from] ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum ShapePart {
    IsPartOf,
    IsNotPartOf,
}

impl TryFrom<char> for ShapePart {
    type Error = ParseShapePartError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::IsNotPartOf),
            '#' => Ok(Self::IsPartOf),
            _ => Err(ParseShapePartError::Unknown),
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseShapePartError {
    #[error("Unknown shape part")]
    Unknown,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct RegionRequirement {
    size: RegionSize,
    shape_quantity: Box<[u64]>,
}

impl FromStr for RegionRequirement {
    type Err = ParseRegionRequirementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size_str, shape_quantities_str) = s
            .split_once(':')
            .ok_or(ParseRegionRequirementError::MissingDelimiter)?;
        Ok(Self {
            size: size_str.parse()?,
            shape_quantity: shape_quantities_str
                .split_whitespace()
                .map(|quantity_str| quantity_str.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseRegionRequirementError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Failed to parse region size: {0}")]
    ParseRegionSize(#[from] ParseRegionSizeError),
    #[error("Failed to parse shape quantities: {0}")]
    ParseShapeQuantity(#[from] ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct RegionSize {
    width: u64,
    length: u64,
}

impl FromStr for RegionSize {
    type Err = ParseRegionSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (width_str, length_str) = s
            .split_once('x')
            .ok_or(ParseRegionSizeError::MissingDelimiter)?;
        Ok(Self {
            width: width_str.parse()?,
            length: length_str.parse()?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseRegionSizeError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Failed to parse either width or length: {0}")]
    Parse(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Arrange
        let input = include_str!("../input/example.day12");

        // Act
        let part1 = part1(input);

        // Assert
        assert_eq!(part1, 2);
    }
}
