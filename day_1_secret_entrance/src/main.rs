use snafu::{ResultExt, Snafu};
use std::str::Chars;

const INITIAL_CURSOR_POSITION: u32 = 50;
const DIAL_MAX_VALUE: u32 = 99;
const DIAL_BASE: u32 = DIAL_MAX_VALUE + 1;

const PATH_TO_INPUT_ERR_MSG: &str = "The first program parameter must be a path to an input file";

fn main() {
    let path = std::env::args()
        .enumerate()
        .find(|(idx, _)| *idx == 1usize)
        .map(|(_, v)| v)
        .unwrap_or_else(|| panic!("{}", PATH_TO_INPUT_ERR_MSG));

    let input =
        std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{}", PATH_TO_INPUT_ERR_MSG));

    let result = run(&input).unwrap_or_else(|e| panic!("{}", e));

    println!("Password: {}", result);
}

fn run(input: &str) -> Result<u32, Error> {
    let mut cursor = INITIAL_CURSOR_POSITION;
    let mut cursor_points_zero_times = 0u32;
    for (line_idx, line) in input.lines().enumerate() {
        if line.len() == 0 {
            break;
        }

        let rotation = Rotation::try_from(line).context(RotationParsingSnafu { line_idx, line })?;

        match rotation {
            Rotation::Left(val) => {
                let val = val % DIAL_BASE;
                cursor = if val > cursor {
                    DIAL_BASE - val + cursor
                } else {
                    cursor - val
                };
            }
            Rotation::Right(val) => {
                cursor = (cursor + val % DIAL_BASE) % DIAL_BASE;
            }
        }

        if cursor == 0 {
            cursor_points_zero_times += 1;
        }
    }

    Ok(cursor_points_zero_times)
}

#[derive(Debug, PartialEq, Snafu)]
enum Error {
    #[snafu(display("Could not parse rotation from input line {line_idx}: {line}"))]
    RotationParsing {
        source: RotationParsingError,
        line_idx: usize,
        line: String,
    },
}

#[derive(Debug, PartialEq)]
enum Rotation {
    Left(u32),
    Right(u32),
}

const ROTATION_LEFT: char = 'L';
const ROTATION_RIGHT: char = 'R';

#[derive(Debug, PartialEq, Snafu)]
enum RotationParsingError {
    #[snafu(display("Rotation input length is less than 2"))]
    LengthIsTooSmall,
    #[snafu(display("Rotation direction must be either L or R. Got: {direction}"))]
    UnexpectedDirection { direction: char },
    #[snafu(display("Rotation distance must be a positive number"))]
    RotationIsNotNumber,
}

impl TryFrom<&str> for Rotation {
    type Error = RotationParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err(RotationParsingError::LengthIsTooSmall);
        }
        let mut chars = value.chars();

        let direction = chars.next().ok_or(RotationParsingError::LengthIsTooSmall)?;

        match direction {
            ROTATION_LEFT => Ok(Rotation::Left(parse_rotation(chars)?)),
            ROTATION_RIGHT => Ok(Rotation::Right(parse_rotation(chars)?)),
            _ => Err(RotationParsingError::UnexpectedDirection { direction }),
        }
    }
}

fn parse_rotation(mut chars: Chars) -> Result<u32, RotationParsingError> {
    let mut rotation = 0;
    while let Some(c) = chars.next() {
        rotation = rotation * 10
            + c.to_digit(10)
                .ok_or(RotationParsingError::RotationIsNotNumber)?;
    }
    Ok(rotation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::left_positive("L10", Some(Rotation::Left(10)))]
    #[case::right_positive("R123", Some(Rotation::Right(123)))]
    #[should_panic(expected = "Rotation input length is less than 2")]
    #[case::too_short("L", None)]
    #[should_panic(expected = "Rotation direction must be either")]
    #[case::incorrect_direction("W7", None)]
    #[should_panic(expected = "Rotation distance must be a positive number")]
    #[case::incorrect_distance("Lseven", None)]
    fn parse_rotation(#[case] rotation: &str, #[case] expected: Option<Rotation>) {
        let actual = Rotation::try_from(rotation)
            .map_err(|e| e.to_string())
            .unwrap();
        assert_eq!(expected, Some(actual));
    }

    #[rstest]
    #[case::example(EXAMPLE_INPUT, Some(3))]
    fn run(#[case] input: &str, #[case] expected: Option<u32>) {
        let result = super::run(input).map_err(|e| e.to_string()).unwrap();
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
