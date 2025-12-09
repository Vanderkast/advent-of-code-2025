use crate::day_2::DayTwoPart;

#[derive(Debug)]
pub(super) struct PartOne {
    total: u32,
}

impl PartOne {
    pub fn new() -> Self {
        Self { total: 0 }
    }
}

impl DayTwoPart for PartOne {
    fn find_invalid_ids(&mut self, range: (&str, &str)) {
        let (mut left, mut right) = range;

        for r_c in right.chars().rev().enumerate() {

        }
    }

    fn result(&self) -> u32 {
        self.total
    }
}
