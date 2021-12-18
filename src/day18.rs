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

impl PairType {
    fn ldfs(&self) -> PairType {
        match self {
            PairType::Value(v) => PairType::Value(*v),
            PairType::Pair(ref pair) => pair.left.ldfs(),
        }
    }

    fn rdfs(&self) -> PairType {
        match self {
            PairType::Value(v) => PairType::Value(*v),
            PairType::Pair(ref pair) => pair.right.rdfs(),
        }
    }
}

impl Pair {
    fn explode(&self) {
        fn _explode(pair: &Pair, depth: usize) {}
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
                let c = dbg!(s.chars().nth(i).unwrap());
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

impl Solution for Day18 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let pair = Pair::from(input);
        dbg!(pair);

        Box::new(0)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let pair = Pair::from(input);

        Box::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[[1,9],[8,5]]";

    const PART1: &str = "";
    const PART2: &str = "";

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
        // let pair = Pair::from("[[[[[9,8],1],2],3],4]");
        // assert_eq!(pair.explode(), Pair::from("[[[[0,9],2],3],4]"));
        // let pair = Pair::from("[7,[6,[5,[4,[3,2]]]]]");
        // assert_eq!(pair.explode(), Pair::from("[7,[6,[5,[7,0]]]]"))
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
