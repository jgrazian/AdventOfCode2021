use crate::prelude::*;

pub struct Day10 {}

impl Day10 {}

fn parse(input: &str) -> impl Iterator<Item = char> + '_ {
    input.trim().chars()
}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let sum = map_lines(input, parse)
            .map(|mut chunk| {
                let mut stack = Vec::with_capacity(128);
                chunk.find_map(|c| match c {
                    '(' | '[' | '{' | '<' => {
                        stack.push(c);
                        None
                    }
                    ')' | ']' | '}' | '>' => {
                        if let Some(open) = stack.pop() {
                            match (open, c) {
                                ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => None,
                                (_, ')') => Some(3),
                                (_, ']') => Some(57),
                                (_, '}') => Some(1197),
                                (_, '>') => Some(25137),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
            })
            .filter_map(|v| v)
            .sum::<i64>();

        Box::new(sum)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let mut scores = map_lines(input, parse)
            .filter_map(|mut chunk| {
                let mut stack = Vec::with_capacity(128);
                if chunk
                    .find_map(|c| match c {
                        '(' | '[' | '{' | '<' => {
                            stack.push(c);
                            None
                        }
                        ')' | ']' | '}' | '>' => {
                            if let Some(open) = stack.pop() {
                                match (open, c) {
                                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => None,
                                    (_, ')') => Some(3),
                                    (_, ']') => Some(57),
                                    (_, '}') => Some(1197),
                                    (_, '>') => Some(25137),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .is_some()
                {
                    None
                } else {
                    Some(stack)
                }
            })
            .map(|stack| {
                stack
                    .into_iter()
                    .rev()
                    .map(|c| match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    })
                    .fold(0i64, |acc, v| acc * 5 + v)
            })
            .collect::<Vec<_>>();
        scores.sort();

        Box::new(scores[scores.len() / 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    const PART1: &str = "26397";
    const PART2: &str = "288957";

    #[test]
    fn test_part1() {
        let day = Day10 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day10 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
