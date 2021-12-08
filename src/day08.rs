use std::ptr::NonNull;

use crate::prelude::*;

pub struct Day08 {}

impl Day08 {}

fn parse(input: &str) -> ([u8; 10], [u8; 4]) {
    let mut signals = [0; 10];
    let mut outputs = [0; 4];

    // signals array
    let mut signal_outputs = input.split("|");
    signal_outputs
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .zip(signals.iter_mut())
        .for_each(|(sig, byte)| {
            *byte = sig.as_bytes().iter().fold(0, |acc, ascii| {
                // Convert from ascii (97 - 103) to (1 - 7)
                let shift = (ascii & 7) - 1;
                acc + (1 << shift)
            });
        });

    // outputs array
    signal_outputs
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .zip(outputs.iter_mut())
        .for_each(|(sig, byte)| {
            *byte = sig.as_bytes().iter().fold(0, |acc, ascii| {
                // Convert from ascii (97 - 103) to (1 - 7)
                let shift = (ascii & 7) - 1;
                acc + (1 << shift)
            });
        });

    (signals, outputs)
}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let digit_count = map_lines(input, parse)
            .map(|(_, output)| {
                // Count occurances of 1, 4, 7, 8 per input
                output
                    .iter()
                    .filter(|byte| match byte.count_ones() {
                        2 => true,
                        4 => true,
                        3 => true,
                        7 => true,
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>();

        Box::new(digit_count)
    }

    //  0000
    // 5    1
    // 5    1
    //  6666
    // 4    2
    // 4    2
    //  3333
    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let mut sum = 0;

        for (_signal, output) in map_lines(input, parse) {
            let mut segment_map = [0u8; 7];
            let mut digit_map = [0u8; 10];

            let mut signals = _signal.to_vec();
            while signals.len() > 0 {
                signals = signals
                    .into_iter()
                    .filter(|byte| match byte.count_ones() {
                        2 => {
                            digit_map[1] = *byte;
                            false
                        }
                        4 => {
                            digit_map[4] = *byte;
                            false
                        }
                        3 => {
                            digit_map[7] = *byte;
                            false
                        }
                        7 => {
                            digit_map[8] = *byte;
                            false
                        }
                        6 => {
                            // Must have 1, 4, 7 and 8 found
                            if digit_map[1] == 0
                                || digit_map[4] == 0
                                || digit_map[7] == 0
                                || digit_map[8] == 0
                            {
                                return true;
                            }

                            let abcfg = digit_map[4] | digit_map[7]; // 9ish shape
                            if ((abcfg | byte) ^ digit_map[8]).count_ones() == 1 {
                                segment_map[3] = abcfg ^ byte;
                                segment_map[4] = (abcfg | byte) ^ digit_map[8];

                                digit_map[9] = *byte;
                                return false;
                            }

                            if ((digit_map[1] | byte) ^ digit_map[8]).count_ones() == 1 {
                                segment_map[6] = (digit_map[1] | byte) ^ digit_map[8];
                                segment_map[5] = digit_map[4] ^ (digit_map[1] | segment_map[6]);

                                digit_map[0] = *byte;
                                return false;
                            }

                            segment_map[0] = digit_map[1] ^ digit_map[7];
                            segment_map[1] = byte ^ digit_map[8];
                            segment_map[2] = digit_map[1] ^ segment_map[1];

                            digit_map[6] = *byte;
                            false
                        }
                        5 => {
                            if *byte
                                == segment_map[0]
                                    | segment_map[1]
                                    | segment_map[3]
                                    | segment_map[4]
                                    | segment_map[6]
                            {
                                digit_map[2] = *byte;
                                return false;
                            }

                            if *byte
                                == segment_map[0]
                                    | segment_map[1]
                                    | segment_map[2]
                                    | segment_map[3]
                                    | segment_map[6]
                            {
                                digit_map[3] = *byte;
                                return false;
                            }

                            if *byte
                                == segment_map[0]
                                    | segment_map[2]
                                    | segment_map[3]
                                    | segment_map[5]
                                    | segment_map[6]
                            {
                                digit_map[5] = *byte;
                                return false;
                            }

                            true
                        }
                        _ => true,
                    })
                    .collect();
            }

            sum += output
                .iter()
                .map(|o| {
                    digit_map.iter().enumerate().find_map(|(digit, val)| {
                        if *val == *o {
                            Some(digit)
                        } else {
                            None
                        }
                    })
                })
                .rev()
                .enumerate()
                .fold(0, |acc, (pos, digit)| {
                    acc + 10usize.pow(pos as u32) * digit.unwrap()
                });
        }

        Box::new(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    const PART1: &str = "26";
    const PART2: &str = "61229";

    #[test]
    fn test_part1() {
        let day = Day08 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day08 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
