#![allow(unused_imports)]
use std::collections::HashMap;

use utils::{Solution, Input};

fn simulate(fish: Vec<i64>, days: i64) -> usize {
    let mut cycles = vec![0, 0, 0, 0, 0, 0, 0];
    for f in &fish {
        cycles[*f as usize] += 1;
    }

    let mut skip = vec![0, 0, 0, 0, 0, 0, 0];
    for i in 0..days {
        let today_cycle = (i % 7) as usize;

        let num_new_offspring = cycles[today_cycle] - skip[today_cycle];
        skip[today_cycle] = 0;

        let new_cycle = ((i + 2) % 7) as usize;
        cycles[new_cycle] += num_new_offspring;

        skip[new_cycle] = num_new_offspring;
    }

    cycles.iter().sum()

}

pub fn solve(input: Input) -> Solution {
    let fish: Vec<i64> = input.raw.split(',').map(|i| i.trim().parse().unwrap()).collect();
    Solution::new(simulate(fish.clone(), 80), simulate(fish.clone(), 256))
}
