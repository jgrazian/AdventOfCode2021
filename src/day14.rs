use crate::prelude::*;

pub struct Day14 {}

impl Day14 {}

fn parse_polymer(input: &str) -> (Vec<u8>, HashMap<&[u8], &[u8]>) {
    let mut lines = input.lines();
    let seed = lines.next().unwrap().as_bytes().to_owned();

    let rules: HashMap<&[u8], &[u8]> = lines
        .filter_map(|line| line.trim().split_once(" -> "))
        .map(|(a, b)| (a.trim().as_bytes(), b.trim().as_bytes()))
        .collect();

    (seed, rules)
}

impl Solution for Day14 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (mut polymer, rules) = parse_polymer(input);

        for _step in 0..10 {
            let mut new_polymer = Vec::new();

            polymer.windows(2).for_each(|window| {
                new_polymer.push(window[0]);
                if let Some(v) = rules.get(window) {
                    new_polymer.push(v[0]);
                }
            });
            new_polymer.push(*polymer.last().unwrap());

            polymer = new_polymer;
        }

        let mut counts = HashMap::new();
        polymer
            .iter()
            .for_each(|v| *counts.entry(v).or_insert(0) += 1);

        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();

        Box::new(max - min)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let (polymer, rules) = parse_polymer(input);
        let mut counts = HashMap::new();
        polymer
            .iter()
            .for_each(|v| *counts.entry(v).or_insert(0) += 1);

        let mut pair_counts = HashMap::new();
        polymer.windows(2).for_each(|v| {
            let key = [v[0], v[1]];
            *pair_counts.entry(key).or_insert(0i64) += 1
        });

        for _step in 0..40 {
            let mut next_pairs = pair_counts.clone();

            let pairs = pair_counts.keys().map(|k| *k).collect::<Vec<_>>();
            for pair in pairs {
                if let Some(v) = rules.get(pair.as_slice()) {
                    let pair_count = *pair_counts.get(&pair).unwrap();

                    *next_pairs.entry([pair[0], v[0]]).or_insert(0) += pair_count;
                    *next_pairs.entry([v[0], pair[1]]).or_insert(0) += pair_count;
                    *next_pairs.entry(pair).or_insert(0) -= pair_count;
                    *counts.entry(&v[0]).or_insert(0) += pair_count;
                }
            }

            pair_counts = next_pairs;
        }

        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();

        Box::new(max - min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    const PART1: &str = "1588";
    const PART2: &str = "2188189693529";

    #[test]
    fn test_part1() {
        let day = Day14 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day14 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
