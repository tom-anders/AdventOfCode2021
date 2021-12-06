#![allow(unused_imports)]
use std::collections::HashMap;

use utils::{Solution, Input};

fn simulate(fish: Vec<i64>, days: i64) -> i64 {
    let mut ages: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for f in &fish {
        ages[*f as usize] += 1;
    }

    for _ in 0..days {
        ages.rotate_left(1);
        ages[6] += *ages.last().unwrap();
    }

    ages.iter().sum()
}

pub fn solve(input: Input) -> Solution {
    let fish: Vec<i64> = input.raw.split(',').map(|i| i.trim().parse().unwrap()).collect();
    Solution::new(simulate(fish.clone(), 80), simulate(fish.clone(), 256))
    // Solution::new(simulate(fish.clone(), 18), "")
}
