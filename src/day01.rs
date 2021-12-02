use crate::prelude::*;

pub struct Day01 {}

impl Day01 {}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, parse_i64);

        let num_increase = parsed
            .iter()
            .zip(parsed.iter().skip(1))
            .filter(|(prev, cur)| *cur - *prev > 0)
            .count();

        Box::new(num_increase)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, parse_i64);

        let window_sums = parsed.windows(3).map(|w| w.iter().sum::<i64>());

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

    const INPUT: &str = r#"199
    200
    208
    210
    200
    207
    240
    269
    260
    263"#;

    const PART1: &str = "7";
    const PART2: &str = "5";

    #[test]
    fn test_part1() {
        let day = Day01 {};
        assert_eq!(day.part1(&INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day01 {};
        assert_eq!(day.part2(&INPUT).to_string(), PART2);
    }
}
