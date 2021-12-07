use crate::prelude::*;

pub struct Day07 {}

impl Day07 {}

fn parse_crabs(input: &str) -> impl Iterator<Item = i64> + '_ {
    input.trim().split(",").map(|s| s.parse().unwrap())
}

impl Solution for Day07 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let mut crab_map = HashMap::new();
        parse_crabs(input).for_each(|loc| {
            let slope = crab_map.entry(loc).or_insert(0);
            *slope += 1;
        });

        let (min, max) = crab_map.keys().fold((i64::MAX, i64::MIN), |acc, v| {
            (*v.min(&acc.0), *v.max(&acc.1))
        });

        let min_energy = (min..=max)
            .map(|target| {
                let min_energy = crab_map
                    .iter()
                    .map(|(loc, slope)| {
                        let dist = (loc - target).abs();
                        slope * dist
                    })
                    .sum::<i64>();
                min_energy
            })
            .min()
            .unwrap();

        Box::new(min_energy)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let mut crab_map = HashMap::new();
        parse_crabs(input).for_each(|loc| {
            let slope = crab_map.entry(loc).or_insert(0);
            *slope += 1;
        });

        let (min, max) = crab_map.keys().fold((i64::MAX, i64::MIN), |acc, v| {
            (*v.min(&acc.0), *v.max(&acc.1))
        });

        fn calc_fuel(n: i64) -> i64 {
            (0..=n).map(|i| i).sum()
        }

        let min_energy = (min..=max)
            .map(|target| {
                let min_energy = crab_map
                    .iter()
                    .map(|(loc, slope)| {
                        let dist = (loc - target).abs();
                        slope * calc_fuel(dist)
                    })
                    .sum::<i64>();
                min_energy
            })
            .min()
            .unwrap();

        Box::new(min_energy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    const PART1: &str = "37";
    const PART2: &str = "168";

    #[test]
    fn test_part1() {
        let day = Day07 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day07 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
