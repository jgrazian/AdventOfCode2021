use crate::prelude::*;

pub struct Day01 {}

impl Day01 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Day01 {
    fn parse(&mut self, input: &str) {}

    fn part1(&self) -> Box<dyn ToString> {
        Box::new(0)
    }

    fn part2(&self) -> Box<dyn ToString> {
        Box::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut day = Day01::new();
        day.parse("");
        assert_eq!(day.part1().to_string(), "");
    }

    #[test]
    fn test_part2() {
        let mut day = Day01::new();
        day.parse("");
        assert_eq!(day.part2().to_string(), "");
    }
}
