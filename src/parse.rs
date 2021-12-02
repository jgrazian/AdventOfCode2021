pub fn map_lines<O>(input: &str, parser: impl Fn(&str) -> O) -> Vec<O> {
    input.lines().map(|i| parser(i)).collect::<Vec<_>>()
}

pub fn parse_i64(input: &str) -> i64 {
    i64::from_str_radix(input, 10).expect("Unable to parse i64.")
}
