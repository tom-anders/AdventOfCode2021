use utils::{Input, Solution};

fn count_increase(v: &Vec<i64>, skip: usize) -> usize {
    v.iter()
        .zip(v.iter().skip(skip))
        .filter(|(lhs, rhs)| rhs > lhs)
        .count()
}

pub fn solve(input: Input) -> Solution {
    let report = input.parse();

    Solution::new(count_increase(&report, 1), count_increase(&report, 3))
}
