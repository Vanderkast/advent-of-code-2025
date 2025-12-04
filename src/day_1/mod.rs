use crate::Part;
use crate::day_1::part_1::PartOne;
use crate::day_1::rotation::{Rotation, RotationParsingError};
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

pub mod part_1;

mod rotation;

const INITIAL_CURSOR_POSITION: u32 = 50;
const DIAL_MAX_VALUE: u32 = 99;
const DIAL_BASE: u32 = DIAL_MAX_VALUE + 1;

#[derive(Debug)]
pub struct DayOne {
    pub part: Part,
    pub path: PathBuf,
}

impl DayOne {
    pub fn run(&self) -> Result<String, String> {
        let input = std::fs::read_to_string(&self.path).map_err(|err| err.to_string())?;
        let part = match self.part {
            Part::PartOne => PartOne::new(),
            Part::PartTwo => {
                todo!()
            }
        };
        let result = Self::run_inner(&input, part).map_err(|err| err.to_string())?;

        Ok(format!("Password: {}", result))
    }

    fn run_inner(input: &str, mut part: impl DayOnePart) -> Result<u32, Error> {
        for (line_idx, line) in input.lines().enumerate() {
            if line.len() == 0 {
                break;
            }

            let rotation =
                Rotation::try_from(line).context(RotationParsingSnafu { line_idx, line })?;

            part.apply(rotation)
        }

        Ok(part.result())
    }
}

pub(super) trait DayOnePart {
    fn apply(&mut self, rotation: Rotation);

    fn result(&self) -> u32;
}

#[derive(Debug, PartialEq, Snafu)]
pub enum Error {
    #[snafu(display("Could not parse rotation from input line {line_idx}: {line}"))]
    RotationParsing {
        source: RotationParsingError,
        line_idx: usize,
        line: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::example(EXAMPLE_INPUT, PartOne::new(), Some(3))]
    fn run_inner(
        #[case] input: &str,
        #[case] part: impl DayOnePart,
        #[case] expected: Option<u32>,
    ) {
        let result = DayOne::run_inner(input, part)
            .map_err(|e| e.to_string())
            .unwrap();
        assert_eq!(expected, Some(result));
    }

    const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
}
