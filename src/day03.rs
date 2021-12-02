use crate::prelude::*;

pub struct Day03 {}

impl Day03 {}

fn parse(input: &str) {}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, parse);

        Box::new(0)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, parse);

        Box::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    const PART1: &str = "";
    const PART2: &str = "";

    #[test]
    fn test_part1() {
        let day = Day03 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day03 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
