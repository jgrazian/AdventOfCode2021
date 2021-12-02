mod parse;

mod day01;
mod day02;
mod day03;

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
        _ => panic!("Expected day number between (1-25)."),
    }
}

pub trait Solution {
    fn part1(&self, input: &str) -> Box<dyn ToString>;
    fn part2(&self, input: &str) -> Box<dyn ToString>;
}
