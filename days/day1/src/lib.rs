fn count_increase(v: &Vec<i64>, skip: usize) -> usize {
    v.iter()
        .zip(v.iter().skip(skip))
        .filter(|(lhs, rhs)| rhs > lhs)
        .count()
}

pub fn solve(input_file: &str) -> utils::Solution {
    let report = utils::get_lines_as(input_file);

    utils::Solution::new(count_increase(&report, 1), count_increase(&report, 3))
}
