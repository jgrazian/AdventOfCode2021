use crate::prelude::*;

pub struct Day15 {}

impl Day15 {}

fn parse_risk_map(input: &str) -> ((usize, usize), Vec<i32>) {
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

impl Solution for Day15 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (size, map) = parse_risk_map(input);

        let mut queue = (0..map.len()).collect::<Vec<_>>();

        let mut dist = vec![99999; map.len()];
        dist[0] = 0;
        let mut prev = vec![99999; map.len()];

        while queue.len() > 0 {
            let (idx, u, dist_u) = queue
                .iter()
                .enumerate()
                .map(|(i, u)| (i, *u, dist[*u]))
                .min_by(|a, b| a.2.cmp(&b.2))
                .unwrap();

            queue.swap_remove(idx);

            if u == map.len() - 1 {
                break;
            }

            for v in get_adjacent(u, size).iter().filter_map(|v| *v) {
                let alt = dist_u + map[v];
                if alt < dist[v] {
                    dist[v] = alt;
                    prev[v] = u;
                }
            }
        }

        let mut sum = 0;
        let mut u = map.len() - 1;
        loop {
            if u == 0 {
                break;
            }
            sum += map[u];
            u = prev[u];
        }

        Box::new(sum)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let (mut size, mut map) = parse_risk_map(input);
        map = map
            .chunks(size.0)
            .flat_map(|w| {
                (0..5)
                    .flat_map(|i| {
                        w.iter()
                            .map(|v| if v + i < 10 { v + i } else { (v + i) % 9 })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        map = (0..5)
            .flat_map(|i| {
                map.iter()
                    .map(|v| if v + i < 10 { v + i } else { (v + i) % 9 })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        size = (size.0 * 5, size.1 * 5);

        let mut queue = (0..map.len()).collect::<Vec<_>>();

        let mut dist = vec![99999; map.len()];
        dist[0] = 0;
        let mut prev = vec![99999; map.len()];

        while queue.len() > 0 {
            let (idx, u, dist_u) = queue
                .iter()
                .enumerate()
                .map(|(i, u)| (i, *u, dist[*u]))
                .min_by(|a, b| a.2.cmp(&b.2))
                .unwrap();

            queue.swap_remove(idx);

            if u == map.len() - 1 {
                break;
            }

            for v in get_adjacent(u, size).iter().filter_map(|v| *v) {
                let alt = dist_u + map[v];
                if alt < dist[v] {
                    dist[v] = alt;
                    prev[v] = u;
                }
            }
        }

        let mut sum = 0;
        let mut u = map.len() - 1;
        loop {
            if u == 0 {
                break;
            }
            sum += map[u];
            u = prev[u];
        }

        Box::new(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    const PART1: &str = "40";
    const PART2: &str = "315";

    #[test]
    fn test_part1() {
        let day = Day15 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day15 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
