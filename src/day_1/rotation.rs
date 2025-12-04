use snafu::Snafu;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Rotation {
    Left(u32),
    Right(u32),
}

const ROTATION_LEFT: char = 'L';
const ROTATION_RIGHT: char = 'R';

#[derive(Debug, PartialEq, Snafu)]
pub enum RotationParsingError {
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
}
