use crate::day_2::{DayTwoPart, Error, NumberParsingSnafu, Range};
use snafu::ResultExt;

#[derive(Debug)]
pub(super) struct PartOne {
    total: u64,
}

impl PartOne {
    pub fn new() -> Self {
        Self { total: 0 }
    }
}

impl DayTwoPart for PartOne {
    fn handle_range(&mut self, range: Range) -> Result<(), Error> {
        let (from, to) = (range.from(), range.to());
        let (from_s, to_s) = (range.original.0, range.original.1);
        if from_s.len() == to_s.len() {
            if from_s.len() & 1 == 0 {
                let a1 = calculate_min(from, &from_s)?;
                let an = calculate_max(to, &to_s)?;
                if a1 <= an {
                    let d = 10u64.pow((from_s.len() >> 1) as u32) + 1;
                    let n = (an - a1) / d + 1;
                    self.total += n * (a1 + an) / 2;
                }
            }
            Ok(())
        } else {
            let max = "9".repeat(from_s.len());
            self.handle_range(Range::new((
                from,
                max.parse().context(NumberParsingSnafu { number: &to_s })?,
            ))?)?;
            for len in from_s.len() + 1..=&to_s.len() - 1 {
                let max = "9".repeat(len);
                self.handle_range(Range::new((
                    10u64.pow((len - 1) as u32),
                    max.parse().context(NumberParsingSnafu { number: max })?,
                ))?)?
            }
            self.handle_range(Range::new((10u64.pow((to_s.len() - 1) as u32), to))?)
        }
    }

    fn result(&self) -> u64 {
        self.total
    }
}

fn calculate_min(from: u64, from_s: &str) -> Result<u64, Error> {
    if from_s.len() & 1 == 0 {
        let a1_l_s = &from_s[0..from_s.len() / 2];
        let a1_l: u64 = a1_l_s
            .parse()
            .context(NumberParsingSnafu { number: a1_l_s })?;
        let a1 = a1_l * 10u64.pow((from_s.len() >> 1) as u32) + a1_l;
        Ok(if a1 >= from {
            a1
        } else {
            (a1_l + 1) * 10u64.pow((from_s.len() >> 1) as u32) + a1_l + 1
        })
    } else {
        Ok(10u64.pow(from_s.len() as u32) + 10u64.pow((from_s.len() >> 1) as u32))
    }
}

fn calculate_max(to: u64, to_s: &str) -> Result<u64, Error> {
    if to < 11 {
        return Ok(0);
    }
    if to_s.len() & 1 == 0 {
        let a1_l_s = &to_s[0..to_s.len() / 2];
        let a1_l: u64 = a1_l_s
            .parse()
            .context(NumberParsingSnafu { number: a1_l_s })?;
        let a1 = a1_l * 10u64.pow((to_s.len() >> 1) as u32) + a1_l;
        Ok(if a1 <= to {
            a1
        } else {
            let mut shift = to_s.len() >> 1;
            if a1 % 10 == 0 {
                shift -= 1;
            }
            (a1_l - 1) * 10u64.pow(shift as u32) + a1_l - 1
        })
    } else {
        let max_s = "9".repeat(to_s.len() - 1);
        max_s.parse().context(NumberParsingSnafu { number: to_s })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(1, 11)]
    #[case(11, 11)]
    #[case(99, 99)]
    #[case(111, 1010)]
    #[case(8998, 9090)]
    #[case(138642, 139139)]
    fn test_calculate_min(#[case] from: u64, #[case] expected: u64) {
        let min = calculate_min(from, from.to_string().as_str()).unwrap();
        assert_eq!(expected, min);
    }

    #[rstest]
    #[case(1, 0)]
    #[case(10, 0)]
    #[case(11, 11)]
    #[case(100, 99)]
    #[case(1000, 99)]
    #[case(9888, 9797)]
    #[case(100000, 9999)]
    #[case(1000000, 999999)]
    fn test_calculate_max(#[case] from: u64, #[case] expected: u64) {
        let max = calculate_max(from, from.to_string().as_str()).unwrap();
        assert_eq!(expected, max);
    }

    #[rstest]
    #[case(&[(11, 11)], 11)]
    #[case(&[(11, 22)], 33)]
    #[case(&[(10, 99)], 495)]
    #[case(&[(1000, 1212)], 3333)]
    #[case(&[(0, 100)], 495)]
    #[case::example(
        &[
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862)
        ],
            1227775554)]
    fn handle_range(#[case] ranges: &[(u64, u64)], #[case] expected: u64) {
        let mut part_one = PartOne::new();
        for range in ranges {
            let range = Range::new(*range).expect("Invalid test input range");
            part_one.handle_range(range).unwrap();
        }
        assert_eq!(expected, part_one.total);
    }
}
