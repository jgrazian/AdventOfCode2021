use crate::prelude::*;

pub struct Day04 {}

impl Day04 {}

fn parse_bingo(input: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut lines = input.lines();

    let order = lines
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::new();

    let mut board = vec![0usize; 25];
    let mut num_filled = 0;
    for line in lines {
        for num in line.trim().split_whitespace() {
            if let Ok(num) = num.parse::<usize>() {
                board[num_filled] = num;
                num_filled += 1;
            }
        }
        if num_filled == 25 {
            boards.push(board);
            board = vec![0usize; 25];
            num_filled = 0;
        }
    }

    (order, boards)
}

fn rows<'a>(board: &'a [usize]) -> impl Iterator<Item = &'a [usize]> + '_ {
    board.chunks(5)
}

fn columns<'a>(board: &'a [usize]) -> impl Iterator<Item = impl Iterator<Item = &usize>> + '_ {
    (0..5).map(move |i| board.iter().skip(i).step_by(5))
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (order, mut boards) = parse_bingo(input);

        for ord in order {
            for board in boards.iter_mut() {
                if let Some(in_board) = board.iter_mut().find(|v| **v == ord) {
                    *in_board = 9999;
                };
            }

            for board in &boards {
                if rows(board).map(|r| r.iter().sum::<usize>()).max() == Some(9999 * 5)
                    || columns(board).map(|c| c.sum::<usize>()).max() == Some(9999 * 5)
                {
                    let sum = board.iter().filter(|v| **v != 9999).sum::<usize>();

                    return Box::new(sum * ord);
                }
            }
        }

        Box::new(0) // unreachable
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let (order, mut boards) = parse_bingo(input);

        let mut remaining_boards = (0..boards.len()).collect::<Vec<_>>();
        for ord in order {
            for board in boards.iter_mut() {
                if let Some(in_board) = board.iter_mut().find(|v| **v == ord) {
                    *in_board = 9999;
                };
            }

            let mut next = Vec::new();
            for id in &remaining_boards {
                let board = &boards[*id];

                if rows(board).map(|r| r.iter().sum::<usize>()).max() == Some(9999 * 5)
                    || columns(board).map(|c| c.sum::<usize>()).max() == Some(9999 * 5)
                {
                    if remaining_boards.len() == 1 {
                        let sum = board.iter().filter(|v| **v != 9999).sum::<usize>();
                        return Box::new(sum * ord);
                    }
                } else {
                    next.push(*id);
                }
            }
            remaining_boards = next;
        }

        Box::new(0) // unreachable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    const PART1: &str = "4512";
    const PART2: &str = "1924";

    #[test]
    fn test_part1() {
        let day = Day04 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day04 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
