use crate::prelude::*;

pub struct Day13 {}

impl Day13 {}

#[derive(Debug)]
enum Fold {
    Up(i32),
    Left(i32),
}

fn parse_folds(input: &str) -> (HashMap<(i32, i32), i32>, Vec<Fold>) {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut folds = Vec::new();

    let mut flag = false;
    for line in input.lines() {
        let line = line.trim();
        if line.len() == 0 {
            flag = true;
            continue;
        }

        if !flag {
            let (left, right) = line.split_once(",").unwrap();
            map.insert((left.parse().unwrap(), right.parse().unwrap()), 1);
        } else {
            let v = line.get(13..).unwrap().parse().unwrap();
            if line.chars().nth(11) == Some('y') {
                folds.push(Fold::Up(v));
            } else {
                folds.push(Fold::Left(v));
            }
        }
    }

    (map, folds)
}

impl Solution for Day13 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (mut map, folds) = parse_folds(input);

        let keys = map.keys().map(|xy| *xy).collect::<Vec<_>>();
        match folds[0] {
            Fold::Up(fold_y) => keys
                .into_iter()
                .filter(|(_, y)| y > &fold_y)
                .for_each(|(x, y)| {
                    map.remove(&(x, y));
                    let dy = fold_y - y;
                    let loc = map.entry((x, fold_y + dy)).or_insert(0);
                    *loc += 1;
                }),
            Fold::Left(fold_x) => {
                keys.into_iter()
                    .filter(|(x, _)| x > &fold_x)
                    .for_each(|(x, y)| {
                        map.remove(&(x, y));
                        let dx = fold_x - x;
                        let loc = map.entry((fold_x + dx, y)).or_insert(0);
                        *loc += 1;
                    })
            }
        }

        let sum = map.values().count();
        Box::new(sum)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let (mut map, folds) = parse_folds(input);

        for fold in folds {
            let keys = map.keys().map(|xy| *xy).collect::<Vec<_>>();

            match fold {
                Fold::Up(fold_y) => {
                    keys.into_iter()
                        .filter(|(_, y)| y > &fold_y)
                        .for_each(|(x, y)| {
                            map.remove(&(x, y));
                            let dy = fold_y - y;
                            let loc = map.entry((x, fold_y + dy)).or_insert(0);
                            *loc += 1;
                        })
                }
                Fold::Left(fold_x) => {
                    keys.into_iter()
                        .filter(|(x, _)| x > &fold_x)
                        .for_each(|(x, y)| {
                            map.remove(&(x, y));
                            let dx = fold_x - x;
                            let loc = map.entry((fold_x + dx, y)).or_insert(0);
                            *loc += 1;
                        })
                }
            }
        }

        let max_x = map.keys().map(|(x, _)| *x).max().unwrap();
        let max_y = map.keys().map(|(_, y)| *y).max().unwrap();

        let mut out = String::new();
        out.push('\n');
        for j in 0..=max_y {
            for i in 0..=max_x {
                out.push(if map.contains_key(&(i, j)) {
                    '█'
                } else {
                    ' '
                });
            }
            out.push('\n');
        }
        Box::new(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    const PART1: &str = "17";
    const PART2: &str = "#####
    #...#
    #...#
    #...#
    #####
    .....
    .....";

    #[test]
    fn test_part1() {
        let day = Day13 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day13 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
