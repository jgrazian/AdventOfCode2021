use crate::prelude::*;

pub struct Day11 {}

impl Day11 {}

fn parse_squid(input: &str) -> ((usize, usize), Vec<i32>) {
    let mut height = 0;

    let map = input
        .lines()
        .flat_map(|l| {
            height += 1;
            l.trim().chars().map(|c| c.to_digit(10).unwrap() as i32)
        })
        .collect::<Vec<_>>();

    let width = map.len() / height;

    ((width, height), map)
}

fn to_coord(index: usize, size: (usize, usize)) -> (usize, usize) {
    (index % size.0, index / size.0)
}

fn to_index(coord: (usize, usize), size: (usize, usize)) -> usize {
    coord.1 * size.0 + coord.0
}

fn get_adjacent(index: usize, size: (usize, usize)) -> [Option<usize>; 8] {
    let coords = to_coord(index, size);
    let indexes = [
        (coords.0 as isize - 1, coords.1 as isize),
        (coords.0 as isize + 1, coords.1 as isize),
        (coords.0 as isize, coords.1 as isize - 1),
        (coords.0 as isize, coords.1 as isize + 1),
        (coords.0 as isize - 1, coords.1 as isize - 1),
        (coords.0 as isize + 1, coords.1 as isize - 1),
        (coords.0 as isize - 1, coords.1 as isize + 1),
        (coords.0 as isize + 1, coords.1 as isize + 1),
    ];
    let mut out = [None; 8];
    out.iter_mut().zip(indexes).for_each(|(v, coord)| {
        if coord.0 < 0 || coord.0 >= size.0 as isize || coord.1 < 0 || coord.1 >= size.1 as isize {
            *v = None;
        } else {
            let coord = (coord.0 as usize, coord.1 as usize);
            *v = Some(to_index(coord, size));
        }
    });

    out
}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (size, mut squid) = parse_squid(input);
        let mut total_flashes = 0;

        for _step in 1..=100 {
            squid.iter_mut().for_each(|s| *s += 1);

            let mut flash_map = vec![false; squid.len()];
            let mut any_flashed = true;
            while any_flashed {
                any_flashed = false; // assume no flashes
                (0..squid.len()).for_each(|i| {
                    if squid[i] > 9 && !flash_map[i] {
                        flash_map[i] = true;
                        any_flashed = true;
                        total_flashes += 1;
                        get_adjacent(i, size)
                            .iter()
                            .filter_map(|v| *v)
                            .for_each(|j| {
                                squid[j] += 1;
                            });
                    }
                });
            }
            squid.iter_mut().for_each(|s| {
                if *s > 9 {
                    *s = 0
                }
            });
        }

        Box::new(total_flashes)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let (size, mut squid) = parse_squid(input);
        let mut total_flashes = 0;

        for _step in 1..=1000 {
            squid.iter_mut().for_each(|s| *s += 1);

            let mut flash_map = vec![false; squid.len()];
            let mut any_flashed = true;
            while any_flashed {
                any_flashed = false; // assume no flashes
                (0..squid.len()).for_each(|i| {
                    if squid[i] > 9 && !flash_map[i] {
                        flash_map[i] = true;
                        any_flashed = true;
                        total_flashes += 1;
                        get_adjacent(i, size)
                            .iter()
                            .filter_map(|v| *v)
                            .for_each(|j| {
                                squid[j] += 1;
                            });
                    }
                });
            }
            if flash_map.iter().filter(|v| **v).count() == 100 {
                return Box::new(_step);
            }
            squid.iter_mut().for_each(|s| {
                if *s > 9 {
                    *s = 0
                }
            });
        }

        Box::new(0) // never reached
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const PART1: &str = "1656";
    const PART2: &str = "195";

    #[test]
    fn test_part1() {
        let day = Day11 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day11 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
