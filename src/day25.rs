use crate::prelude::*;

pub struct Day25 {}

fn parse_cucumbers(input: &str) -> ((u32, u32), Vec<char>) {
    let mut height = 0u32;
    let mut map = Vec::new();
    for line in input.trim().lines() {
        height += 1;

        for c in line.trim().chars() {
            map.push(c);
        }
    }

    let width = map.len() as u32 / height;
    ((width, height), map)
}

fn to_coord(index: usize, size: (u32, u32)) -> (u32, u32) {
    (index as u32 % size.0, index as u32 / size.0)
}

fn to_index(coord: (u32, u32), size: (u32, u32)) -> usize {
    (coord.1 * size.0 + coord.0) as usize
}

fn _draw(map: &Vec<char>, size: (u32, u32)) {
    let mut output = String::new();
    for y in 0..size.1 {
        for x in 0..size.0 {
            let index = to_index((x, y), size);
            output.push(map[index]);
        }
        output.push('\n');
    }
    println!("{}", output);
}

impl Solution for Day25 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let ((width, height), mut map) = parse_cucumbers(input);

        let mut any_moved = true;
        let mut i = 0;
        let mut new_map = map.clone();
        while any_moved {
            new_map.copy_from_slice(&map);
            any_moved = false;

            for y in 0..height {
                for x in 0..width {
                    let index = to_index((x, y), (width, height));
                    match map[index] {
                        '>' => {
                            let mut next_index = index + 1;
                            let (_, next_y) = to_coord(index + 1, (width, height));
                            if next_y > y {
                                next_index = to_index((0, y), (width, height));
                            }
                            if map[next_index] == '>' || map[next_index] == 'v' {
                                continue;
                            }
                            new_map[index] = '.';
                            new_map[next_index] = '>';
                            any_moved = true;
                        }
                        'v' => continue,
                        '.' => continue,
                        _ => panic!("Invalid character: {}", map[index]),
                    }
                }
            }

            map.copy_from_slice(&new_map);

            for y in 0..height {
                for x in 0..width {
                    let index = to_index((x, y), (width, height));
                    match map[index] {
                        '>' => continue,
                        'v' => {
                            let mut next_index = to_index((x, y + 1), (width, height));
                            if y + 1 >= height {
                                next_index = to_index((x, 0), (width, height));
                            }
                            if map[next_index] == '>' || map[next_index] == 'v' {
                                continue;
                            }
                            new_map[index] = '.';
                            new_map[next_index] = 'v';
                            any_moved = true;
                        }
                        '.' => continue,
                        _ => panic!("Invalid character: {}", map[index]),
                    }
                }
            }

            map.copy_from_slice(&new_map);
            i += 1;
        }

        Box::new(i)
    }

    fn part2(&self, _input: &str) -> Box<dyn ToString> {
        Box::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    const PART1: &str = "58";

    #[test]
    fn test_part1() {
        let day = Day25 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }
}
