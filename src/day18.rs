use crate::prelude::*;

pub struct Day18 {}

#[derive(Debug, PartialEq, Clone)]
struct Pair {
    left: PairType,
    right: PairType,
}
#[derive(Debug, PartialEq, Clone)]
enum PairType {
    Value(u32),
    Pair(Box<Pair>),
}

struct PairDfsMut<'a> {
    stack: Vec<(usize, &'a mut PairType)>,
}

impl<'a> Iterator for PairDfsMut<'a> {
    type Item = (usize, &'a mut PairType);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((depth, pair)) = self.stack.pop() {
            match pair {
                PairType::Value(_) => Some((depth, pair)),
                PairType::Pair(p) => {
                    self.stack.push((depth + 1, &mut p.right));
                    self.stack.push((depth + 1, &mut p.left));
                    self.next()
                }
            }
        } else {
            None
        }
    }
}

impl Pair {
    fn _explode(root: &mut Pair) -> bool {
        let mut dfs = PairDfsMut {
            stack: vec![(0, &mut root.right), (0, &mut root.left)],
        };
        let mut prev: Option<&mut PairType> = None;
        let mut seen_left = false;
        while let Some((depth, curr)) = dfs.next() {
            let curr_val = match curr {
                PairType::Value(v) => *v,
                _ => 0,
            };

            if depth == 4 {
                if seen_left {
                    if let Some((_, next)) = dfs.next() {
                        if let PairType::Value(v) = next {
                            *v += curr_val;
                        }
                    }
                    if let PairType::Value(ref mut v) = curr {
                        *v = 99999;
                    }
                    return true;
                } else {
                    if let Some(p) = prev {
                        if let PairType::Value(ref mut v) = p {
                            *v += curr_val;
                        }
                    }
                    seen_left = true;
                    if let PairType::Value(ref mut v) = curr {
                        *v = 99999;
                    }
                }
            }
            prev = Some(curr);
        }

        false
    }

    fn explode(&mut self) -> bool {
        Self::_explode(self);
        let mut stack = vec![&mut self.right, &mut self.left];
        while let Some(p) = stack.pop() {
            if let PairType::Pair(_p) = p.clone() {
                if _p.left == PairType::Value(99999) && _p.right == PairType::Value(99999) {
                    *p = PairType::Value(0);
                    return true;
                }
            }
            if let PairType::Pair(p) = p {
                stack.push(&mut p.right);
                stack.push(&mut p.left);
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        let mut stack = vec![&mut self.right, &mut self.left];
        while let Some(p) = stack.pop() {
            if let PairType::Value(v) = p.clone() {
                if v >= 10 {
                    *p = PairType::Pair(Box::new(Pair {
                        left: PairType::Value((v as f32 / 2.0).floor() as u32),
                        right: PairType::Value((v as f32 / 2.0).ceil() as u32),
                    }));
                    return true;
                }
            }
            if let PairType::Pair(p) = p {
                stack.push(&mut p.right);
                stack.push(&mut p.left);
            }
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            while self.explode() {}
            if !self.split() {
                return;
            }
        }
    }

    fn add(self, other: Self) -> Self {
        Pair {
            left: PairType::Pair(Box::new(self)),
            right: PairType::Pair(Box::new(other)),
        }
    }

    fn sum(&self) -> u32 {
        let left_value = match &self.left {
            PairType::Value(v) => *v,
            PairType::Pair(p) => p.sum(),
        };
        let right_value = match &self.right {
            PairType::Value(v) => *v,
            PairType::Pair(p) => p.sum(),
        };
        3 * left_value + 2 * right_value
    }
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        fn _from(mut s: &str) -> (&str, Pair) {
            s = &s[1..];
            let mut left = None;
            let mut right = None;

            let mut i = 0;
            loop {
                let c = s.chars().nth(i).unwrap();
                match c {
                    '[' => {
                        let (_s, pair) = _from(&s[i..]);
                        if left.is_none() {
                            left = Some(PairType::Pair(Box::new(pair)));
                        } else {
                            right = Some(PairType::Pair(Box::new(pair)));
                        }
                        s = _s;
                        i = 0;
                        continue;
                    }
                    '0'..='9' => {
                        let v = s.chars().nth(i).unwrap().to_digit(10).unwrap() as u32;
                        if left.is_none() {
                            left = Some(PairType::Value(v));
                        } else {
                            right = Some(PairType::Value(v));
                        }
                    }
                    ']' => {
                        return (
                            &s[i + 1..],
                            Pair {
                                left: left.unwrap(),
                                right: right.unwrap(),
                            },
                        )
                    }
                    ',' => (),
                    _ => panic!("unexpected char: {}", s.chars().nth(i).unwrap()),
                }
                i += 1;
            }
        }

        _from(s).1
    }
}

fn parse_pairs(input: &str) -> impl Iterator<Item = Pair> + '_ {
    input.lines().map(|l| Pair::from(l.trim()))
}

impl Solution for Day18 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let reduced = parse_pairs(input)
            .reduce(|acc, item| {
                let mut acc = acc;
                acc = acc.add(item);
                acc.reduce();
                acc
            })
            .unwrap();

        Box::new(reduced.sum())
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let pairs = parse_pairs(input).collect::<Vec<_>>();

        let mut max_sum = 0;
        for i in 0..pairs.len() {
            for j in 0..pairs.len() {
                if i == j {
                    continue;
                }

                let mut acc = pairs[i].clone().add(pairs[j].clone());
                acc.reduce();
                max_sum = max_sum.max(acc.sum());
            }
        }

        Box::new(max_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    const PART1: &str = "4140";
    const PART2: &str = "3993";

    #[test]
    fn test_parse() {
        let pair = Pair::from("[1,2]");
        assert_eq!(
            pair,
            Pair {
                left: PairType::Value(1),
                right: PairType::Value(2)
            }
        );

        let pair = Pair::from("[[1,2],3]");
        assert_eq!(
            pair,
            Pair {
                left: PairType::Pair(Box::new(Pair {
                    left: PairType::Value(1),
                    right: PairType::Value(2)
                })),
                right: PairType::Value(3)
            }
        );

        let pair = Pair::from("[9,[8,7]]");
        assert_eq!(
            pair,
            Pair {
                left: PairType::Value(9),
                right: PairType::Pair(Box::new(Pair {
                    left: PairType::Value(8),
                    right: PairType::Value(7)
                }))
            }
        );

        let pair = Pair::from("[[1,9],[8,5]]");
        assert_eq!(
            pair,
            Pair {
                left: PairType::Pair(Box::new(Pair {
                    left: PairType::Value(1),
                    right: PairType::Value(9)
                })),
                right: PairType::Pair(Box::new(Pair {
                    left: PairType::Value(8),
                    right: PairType::Value(5)
                }))
            }
        );
    }

    #[test]
    fn test_explode() {
        let mut pair = Pair::from("[[[[[9,8],1],2],3],4]");
        pair.explode();
        assert_eq!(pair, Pair::from("[[[[0,9],2],3],4]"));

        let mut pair = Pair::from("[7,[6,[5,[4,[3,2]]]]]");
        pair.explode();
        assert_eq!(pair, Pair::from("[7,[6,[5,[7,0]]]]"));

        let mut pair = Pair::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        pair.explode();
        assert_eq!(pair, Pair::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

        let mut pair = Pair::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        pair.explode();
        assert_eq!(pair, Pair::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn test_split() {
        let mut pair = Pair {
            left: PairType::Value(10),
            right: PairType::Value(2),
        };
        pair.split();
        assert_eq!(pair, Pair::from("[[5,5],2]"));
        let mut pair = Pair {
            left: PairType::Value(11),
            right: PairType::Value(2),
        };
        pair.split();
        assert_eq!(pair, Pair::from("[[5,6],2]"));
    }

    #[test]
    fn test_add_reduce() {
        let mut pair = Pair::from("[[[[4,3],4],4],[7,[[8,4],9]]]");
        pair = pair.add(Pair::from("[1,1]"));
        assert_eq!(pair, Pair::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"));
        pair.reduce();
        assert_eq!(pair, Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_multi_add_reduce() {
        const INPUT: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]";
        let reduced = parse_pairs(INPUT).reduce(|acc, item| {
            let mut acc = acc;
            acc = acc.add(item);
            acc.reduce();
            acc
        });
        assert_eq!(
            reduced.unwrap(),
            Pair::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_part1() {
        let day = Day18 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day18 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
