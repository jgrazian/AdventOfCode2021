use crate::prelude::*;

pub struct Day21 {}

impl Day21 {}

fn parse_players(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let p1 = lines
        .next()
        .map(|s| s.trim()[27..].trim().parse::<u64>().unwrap())
        .unwrap();
    let p2 = lines
        .next()
        .map(|s| s.trim()[27..].trim().parse::<u64>().unwrap())
        .unwrap();

    (p1, p2)
}

fn update_pos(start_pos: u64, dist: u64) -> u64 {
    let out = (start_pos + dist) % 10;
    if out == 0 {
        10
    } else {
        out
    }
}

impl Solution for Day21 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (mut p1_pos, mut p2_pos) = parse_players(input);

        let mut p1_score = 0;
        let mut p2_score = 0;

        let mut dice = 1;
        let mut rolls = 0;

        let mut _loser = 0;
        loop {
            let dist = match dice {
                99 => 99 + 100 + 1,
                100 => 100 + 1 + 2,
                _ => 3 * dice + 3,
            };
            p1_pos = update_pos(p1_pos, dist);
            p1_score += p1_pos;
            dice = match dice {
                99 => 2,
                100 => 3,
                _ => dice + 3,
            };
            rolls += 3;

            if p1_score >= 1000 {
                _loser = p2_score;
                break;
            }

            let dist = match dice {
                99 => 99 + 100 + 1,
                100 => 100 + 1 + 2,
                _ => 3 * dice + 3,
            };
            p2_pos = update_pos(p2_pos, dist);
            p2_score += p2_pos;
            dice = match dice {
                99 => 2,
                100 => 3,
                _ => dice + 3,
            };
            rolls += 3;

            if p2_score >= 1000 {
                _loser = p1_score;
                break;
            }
        }

        Box::new(_loser * rolls)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let (p1_pos, p2_pos) = parse_players(input);

        fn dirac(
            turn: bool,
            p1_pos: u64,
            p2_pos: u64,
            p1_score: u64,
            p2_score: u64,
            universes: u64,
        ) -> (u64, u64) {
            if p1_score >= 21 {
                return (universes, 0);
            }
            if p2_score >= 21 {
                return (0, universes);
            }

            let mut scores = (0, 0);
            if turn {
                for (i, occur) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                    let p1_pos = update_pos(p1_pos, i);
                    let _scores = dirac(
                        false,
                        p1_pos,
                        p2_pos,
                        p1_score + p1_pos,
                        p2_score,
                        universes * occur,
                    );
                    scores = (scores.0 + _scores.0, scores.1 + _scores.1);
                }
            } else {
                for (j, occur) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                    let p2_pos = update_pos(p2_pos, j);
                    let _scores = dirac(
                        true,
                        p1_pos,
                        p2_pos,
                        p1_score,
                        p2_score + p2_pos,
                        universes * occur,
                    );
                    scores = (scores.0 + _scores.0, scores.1 + _scores.1);
                }
            };
            scores
        }

        let (p1_wins, p2_wins) = dirac(true, p1_pos, p2_pos, 0, 0, 1);

        Box::new(p1_wins.max(p2_wins))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Player 1 starting position: 4
    Player 2 starting position: 8";

    const PART1: &str = "739785";
    const PART2: &str = "444356092776315";

    #[test]
    fn test_update_pos() {
        assert_eq!(update_pos(4, 1 + 2 + 3), 10);
        assert_eq!(update_pos(8, 4 + 5 + 6), 3);
        assert_eq!(update_pos(10, 7 + 8 + 9), 4);
        assert_eq!(update_pos(3, 10 + 11 + 12), 6);
    }

    #[test]
    fn test_part1() {
        let day = Day21 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day21 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
