use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let reader = BufReader::new(File::open(input_file).expect(format!("Cannot open {}", input_file).as_str()));
    return reader.lines().map(|line| line.unwrap()).collect();
}

pub fn get_chars(input_file: &str) -> Vec<Vec<char>> {
    get_lines(input_file).iter().map(|line| line.chars().collect()).collect()
}

pub fn get_line(input_file: &str) -> String {
    get_lines(input_file).get(0).unwrap().to_string()
}

pub fn get_lines_as<T: std::str::FromStr>(input_file: &str) -> Vec<T> {
    get_lines(input_file).iter().map(|line| line.parse::<T>().ok().unwrap()).collect()
}
