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

pub struct Input {
    pub raw: String,
}

impl Input {
    pub fn new(input_file: &str) -> Input {
        Input {
            raw: std::fs::read_to_string(input_file).unwrap(),
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.parse()
    }

    pub fn parse<T: std::str::FromStr>(&self) -> Vec<T> {
        self.raw
            .lines()
            .map(|line| line.parse().ok().unwrap())
            .collect()
    }

    pub fn chars(&self) -> Vec<Vec<char>> {
        self.parse::<String>()
            .iter()
            .map(|line| line.chars().collect())
            .collect()
    }

    pub fn get_line(&self, pos: usize) -> String {
        self.raw.lines().nth(pos).unwrap().to_string()
    }
}

