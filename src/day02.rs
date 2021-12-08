use crate::prelude::*;

pub struct Day02 {}

#[derive(Debug)]
enum Direction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn direction_parser(input: &str) -> Direction {
    let split = input.split_whitespace().collect::<Vec<_>>();
    let v = parse_i64(split[1]);
    match split[0] {
        "forward" => Direction::Forward(v),
        "down" => Direction::Down(v),
        "up" => Direction::Up(v),
        _ => panic!("Bad input sting"),
    }
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, direction_parser).collect::<Vec<_>>();

        let mut hor = 0;
        let mut vert = 0;

        parsed.iter().for_each(|d| match d {
            Direction::Forward(c) => hor += c,
            Direction::Down(c) => vert += c,
            Direction::Up(c) => vert -= c,
        });

        Box::new(hor * vert)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, direction_parser).collect::<Vec<_>>();

        let mut aim = 0;
        let mut hor = 0;
        let mut vert = 0;

        parsed.iter().for_each(|d| match d {
            Direction::Forward(c) => {
                hor += c;
                vert += aim * c
            }
            Direction::Down(c) => aim += c,
            Direction::Up(c) => aim -= c,
        });

        Box::new(hor * vert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    const PART1: &str = "150";
    const PART2: &str = "900";

    #[test]
    fn test_part1() {
        let day = Day02 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day02 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
