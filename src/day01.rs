use crate::prelude::*;

pub struct Day01 {
    readings: Vec<i64>,
}

impl Day01 {
    pub fn new() -> Self {
        Self {
            readings: Vec::default(),
        }
    }
}

impl Solution for Day01 {
    fn parse(&mut self, input: &str) {
        fn parse_i64(input: &str) -> IResult<&str, Vec<i64>> {
            many0(terminated(i64, multispace0))(input)
        }

        let (_, readings) = parse_i64(input).expect("Error parsing input!");
        self.readings = readings;
    }

    fn part1(&self) -> Box<dyn ToString> {
        let num_increase = self
            .readings
            .iter()
            .zip(self.readings.iter().skip(1))
            .filter(|(prev, cur)| *cur - *prev > 0)
            .count();

        Box::new(num_increase)
    }

    fn part2(&self) -> Box<dyn ToString> {
        let window_sums = self.readings.windows(3).map(|w| w.iter().sum::<i64>());

        let num_increase = window_sums
            .clone()
            .zip(window_sums.skip(1))
            .filter(|(prev, cur)| *cur - *prev > 0)
            .count();

        Box::new(num_increase)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut day = Day01::new();
        day.parse(
            r#"199
        200
        208
        210
        200
        207
        240
        269
        260
        263"#,
        );
        assert_eq!(day.part1().to_string(), "7");
    }

    #[test]
    fn test_part2() {
        let mut day = Day01::new();
        day.parse(
            r#"199
        200
        208
        210
        200
        207
        240
        269
        260
        263"#,
        );
        assert_eq!(day.part2().to_string(), "5");
    }
}
