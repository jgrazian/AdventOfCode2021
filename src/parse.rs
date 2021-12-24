pub fn map_lines<'a, O>(
    input: &'a str,
    parser: impl Fn(&'a str) -> O + 'a,
) -> impl Iterator<Item = O> + 'a {
    input.trim().lines().map(move |i| parser(i))
}

pub fn parse_i64(input: &str) -> i64 {
    i64::from_str_radix(input.trim(), 10).expect("Unable to parse i64.")
}
