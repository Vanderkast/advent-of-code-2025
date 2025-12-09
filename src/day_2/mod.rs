mod part_1;

use crate::Part;
use crate::day_2::part_1::PartOne;
use snafu::{ResultExt, Snafu};
use std::fmt::Debug;
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct DayTwo {
    pub part: Part,
    pub path: PathBuf,
}

impl DayTwo {
    pub(crate) fn new(part: Part, path: PathBuf) -> Self {
        Self { part, path }
    }

    pub(crate) fn run(&mut self) -> Result<String, String> {
        let input = std::fs::read_to_string(&self.path).map_err(|err| err.to_string())?;

        let result = match self.part {
            Part::PartOne => Self::run_inner(&input, PartOne::new()),
            Part::PartTwo => todo!(),
        }
        .map_err(|err| err.to_string())?;

        Ok(format!("Sum of all invalid IDs: {}", result))
    }

    fn run_inner(input: &str, mut part: impl DayTwoPart) -> Result<u32, Error> {
        for range_split in input.split(",") {
            if let Some((left_num, right_num)) = range_split.split_once("-") {
                left_num.parse::<u32>().context(NumberParsingSnafu {
                    number: left_num,
                    range: range_split,
                })?;
                right_num.parse::<u32>().context(NumberParsingSnafu {
                    number: right_num,
                    range: range_split,
                })?;

                part.find_invalid_ids((left_num, right_num));
            }
        }

        Ok(part.result())
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to parse number {number} in range {range}"))]
    NumberParsing {
        source: ParseIntError,
        number: String,
        range: String,
    },
}

pub(super) trait DayTwoPart: Debug {
    fn find_invalid_ids(&mut self, range: (&str, &str));

    fn result(&self) -> u32;
}

impl From<Part> for Box<dyn DayTwoPart> {
    fn from(part: Part) -> Self {
        Box::new(match part {
            Part::PartOne => PartOne::new(),
            Part::PartTwo => todo!(),
        })
    }
}
