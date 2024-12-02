use std::fs;

pub fn get_input(day: u32) -> String {
    return fs::read_to_string(format!("inputs/{}", day))
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

pub fn convert_strings_to_integers(input: Vec<Vec<String>>) -> Vec<Vec<i32>> {
    return input
        .iter()
        .map(|line| line.iter().map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();
}
