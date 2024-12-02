use std::fs;
use std::str::FromStr;

pub fn get_input(day: u32, small: bool) -> String {
    let suffix = if small { "_small" } else { "" };
    return fs::read_to_string(format!("inputs/{}{}", day, suffix))
        .unwrap_or_else(|_| panic!("no puzzle input for day {} found", day));
}

pub fn parse_input_lines_into_pairs(input: String) -> Vec<(i32, i32)> {
    return input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a = parts.next().unwrap().parse::<i32>().unwrap();
            let b = parts.next().unwrap().parse::<i32>().unwrap();
            return (a, b);
        })
        .collect();
}

pub fn parse_input_matrix<T: FromStr>(input: String) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<T>().unwrap())
                .collect()
        })
        .collect()
}
