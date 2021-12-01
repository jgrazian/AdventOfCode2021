mod day01;

use std::env;
use std::fs;
use std::time::Instant;

mod prelude {
    pub use crate::Solution;
    pub use nom::{
        character::complete::i64,
        character::complete::{multispace0, newline},
        multi::many0,
        sequence::terminated,
        Finish, IResult,
    };
    pub use std::collections::HashMap;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u32 = u32::from_str_radix(&args[1], 10).expect("Expected a day number (1-25)");

    println!("Running Day {:02}:", day);
    let mut solution = get_solution(day);
    let input = fs::read_to_string(format!("./inputs/day{:02}.txt", day))
        .expect("Something went wrong reading the file");

    solution.parse(&input);
    let pt1_start = Instant::now();
    let part1 = solution.part1();
    println!("  Part 1: {}μs", pt1_start.elapsed().as_micros());
    println!("    {} ", part1.to_string(),);

    let pt2_start = Instant::now();
    let part2 = solution.part2();
    println!("  Part 2: {}μs", pt2_start.elapsed().as_micros());
    println!("    {} ", part2.to_string(),);
}

fn get_solution(day: u32) -> Box<dyn Solution> {
    Box::new(match day {
        1 => day01::Day01::new(),
        _ => panic!("Expected day number between (1-25)."),
    })
}

pub trait Solution {
    fn parse(&mut self, input: &str);
    fn part1(&self) -> Box<dyn ToString>;
    fn part2(&self) -> Box<dyn ToString>;
}
