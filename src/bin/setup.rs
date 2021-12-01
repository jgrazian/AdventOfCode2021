use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let day: u32 = u32::from_str_radix(&args[1], 10).expect("Expected a day number (1-25)");

    let _file = File::create(format!("./inputs/day{:02}.txt", day))?;

    let mut file = File::create(format!("./src/day{:02}.rs", day))?;
    file.write_all(
        format!(
            r#"use crate::prelude::*;

pub struct Day{day_name:02} {{}}

impl Day{day_name:02} {{
    pub fn new() -> Self {{
        Self {{}}
    }}
}}

impl Solution for Day{day_name:02} {{
    fn parse(&mut self, input: &str) {{}}

    fn part1(&self) -> Box<dyn ToString> {{
        Box::new(0)
    }}

    fn part2(&self) -> Box<dyn ToString> {{
        Box::new(0)
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_part1() {{
        let mut day = Day{day_name:02}::new();
        day.parse("");
        assert_eq!(day.part1().to_string(), "");
    }}

    #[test]
    fn test_part2() {{
        let mut day = Day{day_name:02}::new();
        day.parse("");
        assert_eq!(day.part2().to_string(), "");
    }}
}}"#,
            day_name = day
        )
        .as_bytes(),
    )?;
    Ok(())
}
