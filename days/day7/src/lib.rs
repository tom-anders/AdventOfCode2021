#![allow(unused_imports)]
use utils::{Input, Solution};

fn find_min(crabs: &[i64], calc_fuel: fn(&i64, &i64) -> i64) -> i64 {
    (*crabs.iter().min().unwrap()..*crabs.iter().max().unwrap() + 1)
        .map(|i| crabs.iter().map(|pos| calc_fuel(pos, &i)).sum())
        .min()
        .unwrap()
}

pub fn solve(input: Input) -> Solution {
    let crabs: Vec<i64> = input.as_vec();

    Solution::new(
        find_min(&crabs, |pos, i| (pos - i).abs()),
        find_min(&crabs, |pos, i| {
            let dist = (pos - i).abs();
            (dist * (dist + 1)) / 2
        }),
    )
}
