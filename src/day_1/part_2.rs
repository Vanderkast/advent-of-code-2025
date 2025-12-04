use crate::day_1::rotation::Rotation;
use crate::day_1::{DIAL_BASE, DayOnePart, INITIAL_CURSOR_POSITION};

pub(super) struct PartTwo {
    cursor: u32,
    cursor_points_zero_times: u32,
}

impl PartTwo {
    pub fn new() -> Self {
        Self {
            cursor: INITIAL_CURSOR_POSITION,
            cursor_points_zero_times: 0,
        }
    }
}

impl DayOnePart for PartTwo {
    fn apply(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left(distance) => {
                self.cursor_points_zero_times += distance / DIAL_BASE;
                let distance = distance % DIAL_BASE;
                self.cursor = if distance > self.cursor {
                    if self.cursor != 0 {
                        self.cursor_points_zero_times += 1;
                    }
                    DIAL_BASE - distance + self.cursor
                } else if distance < self.cursor {
                    self.cursor - distance
                } else {
                    self.cursor_points_zero_times += 1;
                    0
                };
            }
            Rotation::Right(distance) => {
                self.cursor_points_zero_times += (self.cursor + distance) / DIAL_BASE;
                // self.cursor_points_zero_times += val / DIAL_BASE;
                let val = distance % DIAL_BASE;
                self.cursor = (self.cursor + val) % DIAL_BASE;
            }
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
    #[case(Rotation::Left(51), 99, 1)]
    #[case(Rotation::Right(51), 1, 1)]
    #[case(Rotation::Left(50), 0, 1)]
    #[case(Rotation::Right(50), 0, 1)]
    #[case(Rotation::Right(100), 50, 1)]
    fn apply_rotation(
        #[case] rotation: Rotation,
        #[case] expected_cursor: u32,
        #[case] expected_result: u32,
    ) {
        let mut part = PartTwo::new();
        part.apply(rotation);

        assert_eq!(expected_cursor, part.cursor);
        assert_eq!(expected_result, part.result());
    }
}
