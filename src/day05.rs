use crate::prelude::*;

pub struct Day05 {}

impl Day05 {}

fn parse_vents(input: &str) -> ((usize, usize), (usize, usize)) {
    let mut points = input.split(" -> ");

    let mut xy = points.next().unwrap().split(",");
    let p1 = (
        xy.next().unwrap().parse::<usize>().unwrap(),
        xy.next().unwrap().parse::<usize>().unwrap(),
    );

    let mut xy = points.next().unwrap().split(",");
    let p2 = (
        xy.next().unwrap().parse::<usize>().unwrap(),
        xy.next().unwrap().parse::<usize>().unwrap(),
    );

    (p1, p2)
}

fn expand_line(p1: (usize, usize), p2: (usize, usize)) -> Vec<(usize, usize)> {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let dx = match (x1, x2) {
        (x1, x2) if x1 < x2 => 1,
        (x1, x2) if x1 > x2 => -1,
        _ => 0,
    };
    let dy = match (y1, y2) {
        (y1, y2) if y1 < y2 => 1,
        (y1, y2) if y1 > y2 => -1,
        _ => 0,
    };

    let mut x = x1;
    let mut y = y1;
    let mut points = vec![];

    while x != x2 || y != y2 {
        points.push((x, y));
        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
    }
    points.push((x, y));
    points
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, parse_vents);

        let mut map = HashMap::<(usize, usize), usize>::new();
        for (p1, p2) in parsed {
            if p1.0 != p2.0 && p1.1 != p2.1 {
                continue;
            }
            for (x, y) in expand_line(p1, p2) {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }

        let num_crossing = map.iter().filter(|(_, &v)| v >= 2).count();

        Box::new(num_crossing)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let parsed = map_lines(input, parse_vents);

        let mut map = HashMap::<(usize, usize), usize>::new();
        for (p1, p2) in parsed {
            for (x, y) in expand_line(p1, p2) {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }

        let num_crossing = map.iter().filter(|(_, &v)| v >= 2).count();

        Box::new(num_crossing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    const PART1: &str = "5";
    const PART2: &str = "12";

    #[test]
    fn test_part1() {
        let day = Day05 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day05 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
