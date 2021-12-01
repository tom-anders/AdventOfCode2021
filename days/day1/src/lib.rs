fn count_increase(v: &Vec<i64>) -> usize {
    v.iter()
        .zip(v.iter().skip(1))
        .map(|(lhs, rhs)| lhs - rhs)
        .filter(|d| d < &0)
        .count()
}

pub fn solve(input_file: &str) -> utils::Solution {
    let report: Vec<i64> = std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let part2: Vec<i64> = report
        .iter()
        .zip(report.iter().skip(1))
        .zip(report.iter().skip(2))
        .map(|((x, y), z)| x + y + z)
        .collect();

    utils::Solution::new(count_increase(&report), count_increase(&part2))
}
