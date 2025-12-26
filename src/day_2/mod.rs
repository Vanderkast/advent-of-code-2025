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

    fn run_inner(input: &str, mut part: impl DayTwoPart) -> Result<u64, Error> {
        for range_split in input.split(",") {
            if let Some(range) = range_split.split_once("-") {
                let range = Range::try_from(range)?;
                part.handle_range(range)?;
            }
        }
        Ok(part.result())
    }
}

#[derive(Debug, Snafu)]
pub(super) enum Error {
    #[snafu(display("Failed to parse number {number}"))]
    NumberParsing {
        source: ParseIntError,
        number: String,
    },
    #[snafu(display("Range left bound is higher then right bound: {from}-{to}"))]
    RangeBounds { from: u64, to: u64 },
}

pub(super) struct Range {
    range: (u64, u64),
    original: (String, String),
}

impl Range {
    pub fn new(range: (u64, u64)) -> Result<Self, Error> {
        if range.0 <= range.1 {
            Ok(Self {
                range,
                original: (range.0.to_string(), range.1.to_string()),
            })
        } else {
            Err(Error::RangeBounds {
                from: range.0,
                to: range.1,
            })
        }
    }

    pub fn from(&self) -> u64 {
        self.range.0
    }

    pub fn to(&self) -> u64 {
        self.range.1
    }
}

impl<T: AsRef<str>> TryFrom<(T, T)> for Range {
    type Error = Error;

    fn try_from(value: (T, T)) -> Result<Self, Self::Error> {
        let (from, to) = (value.0.as_ref(), value.1.as_ref());
        let from = from
            .parse::<u64>()
            .context(NumberParsingSnafu { number: from })?;
        let to = to
            .parse::<u64>()
            .context(NumberParsingSnafu { number: to })?;
        if to < from {
            Err(Error::RangeBounds { from, to })
        } else {
            Ok(Self {
                range: (from, to),
                original: (from.to_string(), to.to_string()),
            })
        }
    }
}

pub(super) trait DayTwoPart: Debug {
    fn handle_range(&mut self, range: Range) -> Result<(), Error>;

    fn result(&self) -> u64;
}

impl From<Part> for Box<dyn DayTwoPart> {
    fn from(part: Part) -> Self {
        Box::new(match part {
            Part::PartOne => PartOne::new(),
            Part::PartTwo => todo!(),
        })
    }
}
