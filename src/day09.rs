use crate::prelude::*;

pub struct Day09 {}

impl Day09 {}

fn parse_height_map(input: &str) -> ((usize, usize), Vec<u8>) {
    let mut height = 0;

    let map = input
        .lines()
        .flat_map(|l| {
            height += 1;
            l.trim().chars().map(|c| c.to_digit(10).unwrap() as u8)
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

fn get_adjacent(index: usize, size: (usize, usize)) -> [Option<usize>; 4] {
    let coords = to_coord(index, size);
    let indexes = [
        (coords.0 as isize - 1, coords.1 as isize),
        (coords.0 as isize + 1, coords.1 as isize),
        (coords.0 as isize, coords.1 as isize - 1),
        (coords.0 as isize, coords.1 as isize + 1),
    ];
    let mut out = [None; 4];
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

impl Solution for Day09 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (map_size, map) = parse_height_map(input);
        let mut seen = map.iter().map(|_| false).collect::<Vec<bool>>();
        let mut sum = 0;

        let mut stack = (0..map.len()).collect::<Vec<_>>();
        while let Some(i) = stack.pop() {
            if seen[i] {
                continue;
            }

            let mut has_lower_adj = false;
            for adj in get_adjacent(i, map_size).iter().filter_map(|v| *v) {
                if map[adj] <= map[i] {
                    has_lower_adj = true;
                }
            }

            if !has_lower_adj {
                sum += 1 + map[i] as u32;
            }
            seen[i] = true;
        }

        Box::new(sum)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let (map_size, map) = parse_height_map(input);
        let mut seen = map.iter().map(|_| None).collect::<Vec<Option<usize>>>();
        let mut basin_number = 0;

        fn flood_fill(
            i: usize,
            map: &[u8],
            size: (usize, usize),
            seen: &mut [Option<usize>],
            basin_number: usize,
        ) -> usize {
            if seen[i] != None || map[i] == 9 {
                return 0;
            }

            seen[i] = Some(basin_number);
            for adj in get_adjacent(i, size)
                .iter()
                .filter_map(|v| *v)
                .filter(|v| map[*v] != 9)
            {
                flood_fill(adj, map, size, seen, basin_number);
            }

            1
        }

        for i in 0..map.len() {
            basin_number += flood_fill(i, &map, map_size, &mut seen, basin_number);
        }

        // Count items in each basin then sort
        let mut occurrences = HashMap::new();
        for basin_number in seen {
            if let Some(basin_number) = basin_number {
                *occurrences.entry(basin_number).or_insert(0) += 1;
            }
        }
        let mut kv = occurrences.into_iter().collect::<Vec<_>>();
        kv.sort_by(|a, b| a.1.cmp(&b.1));
        kv.reverse();

        Box::new(kv[0].1 * kv[1].1 * kv[2].1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    const PART1: &str = "15";
    const PART2: &str = "1134";

    #[test]
    fn test_part1() {
        let day = Day09 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day09 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
