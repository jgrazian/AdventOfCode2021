use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let day: u32 = u32::from_str_radix(&args[1], 10).expect("Expected a day number (1-25)");

    if Path::new(&format!("./src/day{:02}.rs", day)).exists() {
        println!("Day {} already exists", day);
        return Ok(());
    }

    let _file = File::create(format!("./inputs/day{:02}.txt", day))?;

    let mut file = File::create(format!("./src/day{:02}.rs", day))?;
    file.write_all(
        format!(
            r#"use crate::prelude::*;

pub struct Day{day_name:02} {{}}

impl Day{day_name:02} {{}}

fn parse(input: &str) {{}}

impl Solution for Day{day_name:02} {{
    fn part1(&self, input: &str) -> Box<dyn ToString> {{
        let parsed = map_lines(input, parse);

        Box::new(0)
    }}

    fn part2(&self, input: &str) -> Box<dyn ToString> {{
        let parsed = map_lines(input, parse);

        Box::new(0)
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const INPUT: &str = "";

    const PART1: &str = "";
    const PART2: &str = "";

    #[test]
    fn test_part1() {{
        let day = Day{day_name:02} {{}};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }}

    #[test]
    fn test_part2() {{
        let day = Day{day_name:02} {{}};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }}
}}"#,
            day_name = day
        )
        .as_bytes(),
    )?;
    Ok(())
}
