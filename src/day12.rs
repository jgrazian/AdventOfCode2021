use crate::prelude::*;

pub struct Day12 {}

impl Day12 {}

fn parse_paths(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.trim().split_once("-").unwrap();
        map.entry(from).or_insert(Vec::new()).push(to);
        map.entry(to).or_insert(Vec::new()).push(from);
    }
    map
}

impl Solution for Day12 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let parsed = parse_paths(input);

        let mut stack = parsed
            .get("start")
            .unwrap()
            .iter()
            .map(|s| vec!["start", *s])
            .collect::<Vec<_>>();
        let mut finished = Vec::new();

        while let Some(path) = stack.pop() {
            if let Some(nexts) = parsed.get(path.last().unwrap()) {
                for next in nexts {
                    let mut path = path.clone();
                    if next == &next.to_lowercase() {
                        if next == &"end" {
                            path.push(*next);
                            finished.push(path);
                        } else if !path.contains(next) {
                            path.push(*next);
                            stack.push(path);
                        }
                    } else {
                        path.push(*next);
                        stack.push(path);
                    }
                }
            }
        }

        Box::new(finished.len())
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let parsed = parse_paths(input);

        let mut stack = parsed
            .get("start")
            .unwrap()
            .iter()
            .map(|s| vec!["start", *s])
            .collect::<Vec<_>>();
        let mut finished = Vec::new();

        while let Some(path) = stack.pop() {
            if let Some(nexts) = parsed.get(path.last().unwrap()) {
                for next in nexts {
                    let mut path = path.clone();
                    if next == &next.to_lowercase() {
                        if next == &"end" {
                            path.push(*next);
                            finished.push(path);
                        } else if next == &"start" {
                        } else {
                            let mut counts = HashMap::new();
                            for s in path.iter().filter(|s| *s == &s.to_lowercase()) {
                                *counts.entry(s).or_insert(0) += 1;
                            }
                            let any_twice = counts.values().any(|&c| c > 1);

                            if !any_twice {
                                path.push(*next);
                                stack.push(path);
                            } else if !path.contains(next) {
                                path.push(*next);
                                stack.push(path);
                            }
                        }
                    } else {
                        path.push(*next);
                        stack.push(path);
                    }
                }
            }
        }

        Box::new(finished.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const PART1_SMALL: &str = "10";
    const PART2_SMALL: &str = "36";

    const INPUT_MEDIUM: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const PART1_MEDIUM: &str = "19";
    const PART2_MEDIUM: &str = "103";

    #[test]
    fn test_part1_small() {
        let day = Day12 {};
        assert_eq!(day.part1(INPUT_SMALL).to_string(), PART1_SMALL);
    }

    #[test]
    fn test_part2_small() {
        let day = Day12 {};
        assert_eq!(day.part2(INPUT_SMALL).to_string(), PART2_SMALL);
    }

    #[test]
    fn test_part1_medium() {
        let day = Day12 {};
        assert_eq!(day.part1(INPUT_MEDIUM).to_string(), PART1_MEDIUM);
    }

    #[test]
    fn test_part2_medium() {
        let day = Day12 {};
        assert_eq!(day.part2(INPUT_MEDIUM).to_string(), PART2_MEDIUM);
    }
}
