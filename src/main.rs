mod parse;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

use std::env;
use std::fs;
use std::time::Instant;

mod prelude {
    pub use crate::parse::*;
    pub use crate::Solution;
    pub use std::collections::HashMap;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u32 = u32::from_str_radix(&args[1], 10).expect("Expected a day number (1-25)");

    println!("Running Day {:02}:", day);
    let solution = get_solution(day);
    let input = fs::read_to_string(format!("./inputs/day{:02}.txt", day))
        .expect("Something went wrong reading the file");

    let pt1_start = Instant::now();
    let part1 = solution.part1(&input);
    println!(
        "  Part 1: {}ms",
        pt1_start.elapsed().as_micros() as f64 / 1000.0
    );
    println!("    {} ", part1.to_string(),);

    let pt2_start = Instant::now();
    let part2 = solution.part2(&input);
    println!(
        "  Part 2: {}ms",
        pt2_start.elapsed().as_micros() as f64 / 1000.0
    );
    println!("    {} ", part2.to_string(),);
}

fn get_solution(day: u32) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day01::Day01 {}),
        2 => Box::new(day02::Day02 {}),
        3 => Box::new(day03::Day03 {}),
        4 => Box::new(day04::Day04 {}),
        5 => Box::new(day05::Day05 {}),
        6 => Box::new(day06::Day06 {}),
        7 => Box::new(day07::Day07 {}),
        8 => Box::new(day08::Day08 {}),
        9 => Box::new(day09::Day09 {}),
        10 => Box::new(day10::Day10 {}),
        11 => Box::new(day11::Day11 {}),
        12 => Box::new(day12::Day12 {}),
        13 => Box::new(day13::Day13 {}),
        14 => Box::new(day14::Day14 {}),
        _ => panic!("Expected day number between (1-25)."),
    }
}

pub trait Solution {
    fn part1(&self, input: &str) -> Box<dyn ToString>;
    fn part2(&self, input: &str) -> Box<dyn ToString>;
}
