use crate::prelude::*;

pub struct Day17 {}

impl Day17 {}

fn parse_target_area(input: &str) -> ((i32, i32), (i32, i32)) {
    let (_, input) = input.split_once("x=").unwrap();
    let (x, y) = input.split_once(", y=").unwrap();
    let (x_min, x_max) = x.split_once("..").unwrap();
    let (y_min, y_max) = y.split_once("..").unwrap();

    (
        (x_min.parse().unwrap(), x_max.parse().unwrap()),
        (y_min.parse().unwrap(), y_max.parse().unwrap()),
    )
}

impl Solution for Day17 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let (_, (y_min, y_max)) = parse_target_area(input);

        let max_height = (-1000..1000)
            .filter_map(|mut dy| {
                let mut y = 0;
                let mut max_y = 0;
                for _t in 0..5000 {
                    y += dy;
                    dy -= 1;

                    if y > max_y {
                        max_y = y;
                    }

                    if y <= y_max && y >= y_min {
                        return Some(max_y);
                    }

                    if dy < 1 && y < y_min {
                        return None;
                    }
                }
                return None;
            })
            .max()
            .unwrap();

        Box::new(max_height)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let ((x_min, x_max), (y_min, y_max)) = parse_target_area(input);

        let mut count = 0;
        for dx0 in -500..500 {
            for dy0 in -500..500 {
                let mut x = 0;
                let mut y = 0;
                let mut dx = dx0;
                let mut dy = dy0;

                loop {
                    x += dx;
                    y += dy;
                    dx = if dx == 0 {
                        0
                    } else if dx > 0 {
                        dx - 1
                    } else {
                        dx + 1
                    };
                    dy -= 1;

                    if x >= x_min && x <= x_max && y <= y_max && y >= y_min {
                        count += 1;
                        break;
                    }
                    if x > x_max || y < y_min {
                        break;
                    }
                }
            }
        }

        Box::new(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    const PART1: &str = "45";
    const PART2: &str = "112";

    #[test]
    fn test_part1() {
        let day = Day17 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day17 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
