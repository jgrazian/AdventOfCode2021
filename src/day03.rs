use crate::prelude::*;

pub struct Day03 {}

impl Day03 {}

fn parse_binary(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Unable to parse char to int."))
        .collect()
}

fn to_decimal(input: &[u32]) -> i64 {
    input
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, bit)| acc + (*bit as i64 * 2i64.pow(i as u32)))
}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, parse_binary).collect::<Vec<_>>();

        let mut gamma = parsed[0].iter().map(|_| 0).collect::<Vec<_>>();

        for line in &parsed {
            for (i, bit) in line.iter().enumerate() {
                if *bit == 1 {
                    gamma[i] += 1;
                }
            }
        }

        gamma.iter_mut().for_each(|bit| {
            let half = (parsed.len() as f32 / 2.0) as u32;

            *bit = if *bit > half { 1 } else { 0 };
        });

        let epsilon = gamma
            .iter()
            .map(|bit| match bit {
                0 => 1,
                1 => 0,
                _ => 999999,
            })
            .collect::<Vec<_>>();

        Box::new(to_decimal(&gamma) * to_decimal(&epsilon))
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, parse_binary).collect::<Vec<_>>();

        let mut oxy = parsed
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let mut co2 = oxy.clone();

        let mut bit_idx = 0;
        while oxy.len() > 1 {
            let count_0_bits = oxy.iter().fold(0, |acc, i| {
                if parsed[*i][bit_idx] == 0 {
                    acc + 1
                } else {
                    acc
                }
            });
            let keep_zeros = count_0_bits > (oxy.len() / 2);

            oxy = oxy
                .into_iter()
                .filter(|i| match (keep_zeros, parsed[*i][bit_idx] == 0) {
                    (true, true) => true,
                    (true, false) => false,
                    (false, true) => false,
                    (false, false) => true,
                })
                .collect();

            bit_idx += 1;
        }
        let oxy_rating = to_decimal(&parsed[oxy[0]]);

        let mut bit_idx = 0;
        while co2.len() > 1 {
            let count_0_bits = co2.iter().fold(0, |acc, i| {
                if parsed[*i][bit_idx] == 0 {
                    acc + 1
                } else {
                    acc
                }
            });
            let keep_zeros = count_0_bits <= (co2.len() / 2);

            co2 = co2
                .into_iter()
                .filter(|i| match (keep_zeros, parsed[*i][bit_idx] == 0) {
                    (true, true) => true,
                    (true, false) => false,
                    (false, true) => false,
                    (false, false) => true,
                })
                .collect();

            bit_idx += 1;
        }
        let co2_rating = to_decimal(&parsed[co2[0]]);

        Box::new(oxy_rating * co2_rating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    const PART1: &str = "198";
    const PART2: &str = "230";

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
