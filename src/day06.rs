use crate::prelude::*;

pub struct Day06 {}

impl Day06 {}

fn parse_fish(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let mut fishes = parse_fish(input);

        for _day in 0..80 {
            for i in 0..fishes.len() {
                fishes[i] -= 1;
                if fishes[i] == -1 {
                    fishes.push(8);
                    fishes[i] = 6;
                }
            }
        }

        Box::new(fishes.len())
    }

    // fn part2(&self, input: &str) -> Box<dyn ToString> {
    //     let mut fish_map: HashMap<i64, i64> = HashMap::new();
    //     parse_fish(input).into_iter().for_each(|k| {
    //         let v = fish_map.entry(k).or_insert(0);
    //         *v += 1;
    //     });

    //     for _day in 0..256 {
    //         let mut next_map = HashMap::new();
    //         for days in fish_map.keys() {
    //             if *days > 0 {
    //                 let v = fish_map.get(days).unwrap(); // Get # of fish at current day
    //                 let next = next_map.entry(days - 1).or_insert(0); // Re-add at day - 1
    //                 *next += *v;
    //             } else {
    //                 let new_fish = next_map.entry(8).or_insert(0);
    //                 let rollover = fish_map.get(days).unwrap(); // Get # of fish at day=-1 (ready to spawn)
    //                 *new_fish += *rollover;

    //                 let next_rollover = next_map.entry(6).or_insert(0);
    //                 *next_rollover += *rollover;
    //             }
    //         }
    //         fish_map = next_map; // Advance to next day
    //     }

    //     Box::new(fish_map.values().sum::<i64>())
    // }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let mut fishes = [0i64; 9];
        input.trim().split(",").for_each(|s| {
            let i = s.parse::<usize>().unwrap();
            fishes[i] += 1;
        });

        for _ in 0..256 {
            let mut next = [0; 9];
            next[0] = fishes[1];
            next[1] = fishes[2];
            next[2] = fishes[3];
            next[3] = fishes[4];
            next[4] = fishes[5];
            next[5] = fishes[6];
            next[6] = fishes[0] + fishes[7];
            next[7] = fishes[8];
            next[8] = fishes[0];
            fishes = next;
        }

        Box::new(fishes.iter().sum::<i64>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    const PART1: &str = "5934";
    const PART2: &str = "26984457539";

    #[test]
    fn test_part1() {
        let day = Day06 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day06 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
