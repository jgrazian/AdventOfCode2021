use crate::prelude::*;

pub struct Day20 {}

impl Day20 {}

fn parse_image(input: &str) -> (Vec<bool>, HashMap<(i64, i64), bool>) {
    let mut lines = input.lines();
    let algorithm = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let image_map = lines
        .filter_map(|line| match line.trim() {
            "" => None,
            _ => Some(line.trim()),
        })
        .enumerate()
        .flat_map(|(j, line)| {
            line.chars().enumerate().map(move |(i, c)| {
                (
                    (i as i64, j as i64),
                    match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    },
                )
            })
        })
        .collect();

    (algorithm, image_map)
}

fn image_bounds(image: &HashMap<(i64, i64), bool>) -> ((i64, i64), (i64, i64)) {
    let mut min_x = 9999;
    let mut max_x = 0;
    let mut min_y = 9999;
    let mut max_y = 0;

    for k in image.keys() {
        min_x = min_x.min(k.0);
        max_x = max_x.max(k.0);
        min_y = min_y.min(k.1);
        max_y = max_y.max(k.1);
    }

    ((min_x, min_y), (max_x, max_y))
}

fn window(
    image: &HashMap<(i64, i64), bool>,
    loc: (i64, i64),
    bounds: ((i64, i64), (i64, i64)),
    boundary: bool,
) -> [bool; 9] {
    let mut out = [false; 9];
    let ((min_x, min_y), (max_x, max_y)) = bounds;

    let mut i = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            let cur_cell = (loc.0 + dx, loc.1 + dy);
            if cur_cell.0 < min_x || cur_cell.0 > max_x || cur_cell.1 < min_y || cur_cell.1 > max_y
            {
                out[i] = boundary;
            } else {
                if let Some(v) = image.get(&cur_cell) {
                    out[i] = *v;
                } else {
                    out[i] = false;
                }
            }
            i += 1;
        }
    }
    out
}

fn to_usize(code: [bool; 9]) -> usize {
    code.iter()
        .rev()
        .enumerate()
        .filter(|(_, v)| **v)
        .fold(0, |acc, (i, _)| acc + 2usize.pow(i as u32))
}

impl Solution for Day20 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (algo, mut image) = parse_image(input);

        let mut boundary = false;
        for _i in 0..2 {
            let mut tmp_image = HashMap::with_capacity(image.len());
            let ((min_x, min_y), (max_x, max_y)) = image_bounds(&image);

            for y in (min_y - 1)..=(max_y + 1) {
                for x in (min_x - 1)..=(max_x + 1) {
                    let loc = (x, y);
                    let window_value = to_usize(window(
                        &mut image,
                        loc,
                        ((min_x, min_y), (max_x, max_y)),
                        boundary,
                    ));
                    let new_value = algo[window_value];

                    tmp_image.insert(loc, new_value);
                }
            }

            boundary = if boundary {
                *algo.last().unwrap()
            } else {
                algo[0]
            };

            image = tmp_image;
        }

        Box::new(image.values().filter(|v| **v).count())
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let (algo, mut image) = parse_image(input);

        let mut boundary = false;
        for _i in 0..50 {
            let mut tmp_image = HashMap::with_capacity(image.len());
            let ((min_x, min_y), (max_x, max_y)) = image_bounds(&image);

            for y in (min_y - 1)..=(max_y + 1) {
                for x in (min_x - 1)..=(max_x + 1) {
                    let loc = (x, y);
                    let window_value = to_usize(window(
                        &mut image,
                        loc,
                        ((min_x, min_y), (max_x, max_y)),
                        boundary,
                    ));
                    let new_value = algo[window_value];

                    tmp_image.insert(loc, new_value);
                }
            }

            boundary = if boundary {
                *algo.last().unwrap()
            } else {
                algo[0]
            };

            image = tmp_image;
        }

        Box::new(image.values().filter(|v| **v).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
    #..#.
    #....
    ##..#
    ..#..
    ..###";

    const PART1: &str = "35";
    const PART2: &str = "3351";

    #[test]
    fn test_part1() {
        let day = Day20 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day20 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
