#![allow(unused_imports)]
use std::collections::HashSet;
use std::iter::FromIterator;

use utils::{Input, Solution};

struct Grid {
    data: Vec<i32>,
    size: i32,
    total_flashes: usize,
}

impl Grid {
    fn new(s: String) -> Grid {
        Grid {
            total_flashes: 0,
            size: s.lines().count() as i32,
            data: s
                .lines()
                .map(|s| {
                    s.chars()
                        .map(|c| c.to_digit(10).unwrap() as i32)
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect(),
        }
    }

    fn neighbor_indices(&self, i: i32) -> Vec<usize> {
        let (row, col) = (i / self.size, i % self.size);
        vec![
            (row - 1, col), (row + 1, col),
            (row, col - 1), (row, col + 1),
            (row + 1, col - 1), (row + 1, col + 1),
            (row - 1, col + 1), (row - 1, col - 1),
        ].iter().filter(|(r, c)| *r >= 0 && *c >= 0 && *r < self.size && *c < self.size)
            .map(|(r, c)| (c + r * self.size) as usize).collect()
    }

    fn iterate(&mut self) -> bool {
        self.data.iter_mut().for_each(|x| *x += 1);

        loop {
            let new_flashes: HashSet<i32> = HashSet::from_iter(
                self.data.iter()
                    .enumerate()
                    .filter(|(i, x)| **x > 9)
                    .map(|(i, _)| i as i32)
            );

            if new_flashes.is_empty() {
                break;
            }

            for i in &new_flashes {
                for n in self.neighbor_indices(*i) {
                    self.data[n] += 1;
                }
            }

            // Mark as flashed
            for i in &new_flashes {
                self.data[*i as usize] = i32::MIN;
            }

            self.total_flashes += &new_flashes.len();
        }

        let mut count = 0;
        self.data.iter_mut().for_each(|x| if *x < 0 { *x = 0; count += 1});

        count == self.size * self.size
    }

    fn print(&self) {
        for (i, x) in self.data.iter().enumerate() {
            print!("{}", x);
            if (i as i32 + 1) % self.size == 0 {
                println!();
            }
        }
        println!();
    }
}

pub fn solve(input: Input) -> Solution {
    let mut grid = Grid::new(input.raw.clone());

    for i in 0..100 {
        grid.iterate();
    }
    let part1 = grid.total_flashes;

    grid = Grid::new(input.raw);
    let mut step = 1; 
    while !grid.iterate() { step += 1; }

    Solution::new(part1, step)
}
