pub struct Solution {
    pub part1: String,
    pub part2: String,
}

impl Solution {
    pub fn new<T1: std::fmt::Display, T2: std::fmt::Display>(p1: T1, p2: T2) -> Solution {
        Solution {
            part1: p1.to_string(),
            part2: p2.to_string(),
        }
    }
}

pub fn get_lines(input_file: &str) -> Vec<String> {
    std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

pub fn get_chars(input_file: &str) -> Vec<Vec<char>> {
    get_lines(input_file)
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn get_line(input_file: &str) -> String {
    get_lines(input_file).first().unwrap().to_string()
}

pub fn get_lines_as<T: std::str::FromStr>(input_file: &str) -> Vec<T> {
    get_lines(input_file)
        .iter()
        .map(|line| line.parse().ok().unwrap())
        .collect()
}
