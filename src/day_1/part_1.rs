use crate::day_1::rotation::Rotation;
use crate::day_1::{DayOnePart, DIAL_BASE, INITIAL_CURSOR_POSITION};

pub(crate) struct PartOne {
    cursor: u32,
    cursor_points_zero_times: u32,
}

impl PartOne {
    pub fn new() -> PartOne {
        Self {
            cursor: INITIAL_CURSOR_POSITION,
            cursor_points_zero_times: 0,
        }
    }
}

impl DayOnePart for PartOne {
    fn apply(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left(val) => {
                let val = val % DIAL_BASE;
                self.cursor = if val > self.cursor {
                    DIAL_BASE - val + self.cursor
                } else {
                    self.cursor - val
                };
            }
            Rotation::Right(val) => {
                self.cursor = (self.cursor + val % DIAL_BASE) % DIAL_BASE;
            }
        }
        if self.cursor == 0 {
            self.cursor_points_zero_times += 1;
        }
    }

    fn result(&self) -> u32 {
        self.cursor_points_zero_times
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Rotation::Left(1), 49, 0)]
    #[case(Rotation::Right(1), 51, 0)]
    #[case(Rotation::Left(51), 99, 0)]
    #[case(Rotation::Right(51), 1, 0)]
    #[case(Rotation::Left(50), 0, 1)]
    #[case(Rotation::Right(50), 0, 1)]
    fn run(#[case] rotation: Rotation, #[case] expected_cursor: u32, #[case] expected_result: u32) {
        let mut part = PartOne::new();
        part.apply(rotation);

        assert_eq!(expected_cursor, part.cursor);
        assert_eq!(expected_result, part.result());
    }
}
